use std::thread;
use std::time::Duration;
use std::mem::drop;
use std::sync::{mpsc, mpsc::{Receiver, Sender}};

use druid::{AppLauncher, PlatformError, WindowDesc};
use druid::im::Vector;

mod models;
mod delegate;
mod ui;

fn main() -> Result<(), PlatformError> {
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
        listener_thread(&rx_data, &tx_status, &rx_disconnect, &tx_status2);
    });

    AppLauncher::with_window(main_window)
        .delegate(delegate::Delegate)
        .log_to_console()
        .launch(data)
}

fn listener_thread(rx_data: &Receiver<models::AppData>, tx_status: &Sender<models::Status>, rx_disconnect: &Receiver<bool>, tx_status2: &Sender<models::Status>) {
    loop {
        println!("Waiting for connection command...");
        let received = rx_data.recv().unwrap();
        let baud_rate = received.baud_rate.parse().expect("Baudrate should be an integer");

        match serialport::new(received.serial_port.clone(), baud_rate).open() {
            Ok(mut p) => {
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
                    match p.read(buf.as_mut_slice()) {
                        Ok(_) => {
                            let c = buf[0];
                            println!("Received data: {c}");

                            for rule in received.clone().rules {
                                let ch = rule.character.chars().next().expect("Empty rule!");
                                if char::from(c) == ch {
                                    println!("Matched rule ({ch})! Sending MIDI message...");
                                }
                            }
                        },
                        Err(_) => {}
                    }
                }
            },
            Err(e) => {
                let status = models::Status {connected: false, message: format!("Error connecting to serial port: {e}")};
                let _ = tx_status.send(status);
            }
        }
    }
}
