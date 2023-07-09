use std::thread;
use std::time::Duration;
use std::mem::drop;
use std::sync::{mpsc, mpsc::{Receiver, Sender}};

use druid::{AppLauncher, PlatformError, WindowDesc};
use druid::im::Vector;
use serialport::SerialPort;

mod models;
mod delegate;
mod ui;

fn main() -> Result<(), PlatformError> {
    let (client, _status) = jack::Client::new("whismur", jack::ClientOptions::NO_START_SERVER).unwrap();
    let mut midi_port = client.register_port("out", jack::MidiOut::default()).expect("Error creating MIDI out port!");
    let (midi_sender, midi_receiver) = mpsc::channel::<models::Midi>();

    let cback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let mut put_p = midi_port.writer(ps);
        if let Ok(midi) = midi_receiver.recv_timeout(Duration::from_millis(1)) {
            put_p.write(&jack::RawMidi {
                time: 0,
                bytes: &[
                    midi.data | midi.channel,
                    midi.note,
                    midi.velocity
                ],
            }).unwrap();
        };
        jack::Control::Continue
    };

    let (tx_data,rx_data) = mpsc::channel::<models::AppData>();
    let (tx_status, rx_status) = mpsc::channel::<models::Status>();
    let (tx_status2, rx_status2) = mpsc::channel::<models::Status>();
    let (tx_disconnect, rx_disconnect) = mpsc::channel::<bool>();

    let main_window = WindowDesc::new(ui::ui_builder(tx_data, rx_status, tx_disconnect, rx_status2))
        .title("Whismur")
        .with_min_size((950.0,420.0))
        .window_size((950.0,420.0));

    let data = models::AppData {
        serial_port: String::from("/dev/ttyACM0"),
        baud_rate: String::from("9600"),
        rules: Vector::new(),
        connected: false
    };

    thread::spawn(move || {
        listener_thread(&rx_data, &tx_status, &rx_disconnect, &tx_status2, midi_sender);
    });

    let activate_client = client
        .activate_async((), jack::ClosureProcessHandler::new(cback))
        .unwrap();

    let result = AppLauncher::with_window(main_window)
        .delegate(delegate::Delegate)
        .log_to_console()
        .launch(data);

    activate_client.deactivate().unwrap();
    result
}

fn connect(data: &models::AppData) -> Result<(Box<dyn SerialPort>, Vector<models::ParsedRule>), String> {
    let baud_rate = match data.baud_rate.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u32>() {
        Ok(b) => b,
        Err(_) => return Err(String::from("Baudrate must be an integer!"))
    };

    let parsed_rules = match models::parse_rules(data.rules.clone()) {
        Ok(p) => p,
        Err(message) => return Err(message)
    };

    match serialport::new(data.serial_port.clone(), baud_rate).open() {
        Ok(p) => Ok((p, parsed_rules)),
        Err(e) => Err(format!("Error connecting to the port: {e}"))
    }
}

fn listener_thread(rx_data: &Receiver<models::AppData>, tx_status: &Sender<models::Status>, rx_disconnect: &Receiver<bool>, tx_status2: &Sender<models::Status>, midi_sender: Sender<models::Midi>) {
    loop {
        println!("Waiting for connection command...");
        let received = rx_data.recv().unwrap();
        match connect(&received) {
            Ok((mut p, rules)) => {
                let status = models::Status {connected: true, message: String::from("")};
                let _ = tx_status.send(status);
                println!("Connected!");

                loop {
                    let status = match rx_disconnect.recv_timeout(Duration::from_millis(1)) {
                        Ok(s) => s,
                        Err(_e) => false
                    };

                    if status {
                        let status = models::Status {connected: false, message: String::from("")};
                        let _ = tx_status2.send(status);
                        drop(p);
                        break;
                    }

                    let mut buf: Vec<u8> = vec![0;1];

                    if p.read(buf.as_mut_slice()).is_ok() {
                        let c = buf[0];
                        println!("Received data: {c}");

                        for rule in rules.clone() {
                            let ch = rule.character;
                            if char::from(c) == ch {
                                let _ = midi_sender.send(models::Midi {channel: rule.channel, note: rule.code, data: rule.data, velocity: rule.velocity});
                                println!("Matched rule ({ch})!");
                            }
                        }
                    }
                }
            },
            Err(e) => {
                let status = models::Status {connected: false, message: e};
                let _ = tx_status.send(status);
                println!("Could not connect!")
            }
        }
    }
}
