use druid::widget::{Flex, Label, TextBox, Button};
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};

mod models;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());

    let data = models::AppData {serial_port: String::from(""), baud_rate: String::from("9600")};

    AppLauncher::with_window(main_window).launch(data)
}

fn ui_builder() -> impl Widget<models::AppData> {
    let serial_port_label = Label::new("Port").padding(10.0).align_left();
    let serial_port_text_box = TextBox::new()
        .with_placeholder("/dev/ttyACM0").lens(models::AppData::serial_port).align_left();

    let baud_rate_label = Label::new("Baud Rate").padding(10.0).align_left();
    let baud_rate_text_box = TextBox::new()
        .with_placeholder("9600").lens(models::AppData::baud_rate);

    let connect_button = Button::new("Connect").padding(10.0).align_right();

    let serial_row = Flex::row()
        .with_child(serial_port_label)
        .with_child(serial_port_text_box)
        .with_child(baud_rate_label)
        .with_child(baud_rate_text_box)
        .with_child(connect_button);

    Flex::column().with_child(serial_row)
}
