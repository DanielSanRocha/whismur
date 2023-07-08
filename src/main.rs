use druid::{AppLauncher, PlatformError, WindowDesc};
use druid::im::Vector;

mod models;
mod delegate;
mod ui;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui::ui_builder())
        .title("Whismur")
        .with_min_size((950.0,420.0))
        .window_size((950.0,420.0));

    let data = models::AppData {
        serial_port: String::from("/dev/ttyACM0"),
        baud_rate: String::from("9600"),
        rules: Vector::new(),
        connected: false
    };

    AppLauncher::with_window(main_window)
        .delegate(delegate::Delegate)
        .log_to_console()
        .launch(data)
}
