use druid::widget::{Flex, Label, TextBox, Button};
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc, FontDescriptor, FontFamily, Color};

mod models;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .with_min_size((950.0,400.0))
        .title("Whismur")
        .window_size((950.0,400.0));

    let data = models::AppData {serial_port: String::from("/dev/ttyACM0"), baud_rate: String::from("9600"), connected: false};

    AppLauncher::with_window(main_window).launch(data)
}

fn ui_builder() -> impl Widget<models::AppData> {
    let font = FontDescriptor::new(FontFamily::MONOSPACE).with_size(26.0);

    let serial_port_label = Label::new("Port")
        .with_font(font.clone())
        .padding(5.0)
        .align_left();
    let serial_port_text_box = TextBox::new()
        .with_font(font.clone())
        .with_placeholder("/dev/ttyACM0")
        .fix_width(350.0)
        .lens(models::AppData::serial_port)
        .padding(5.0)
        .align_left()
        .disabled_if(|data, _| data.connected);

    let baud_rate_label = Label::new("Baud Rate")
        .with_font(font.clone())
        .padding(5.0)
        .align_left();
    let baud_rate_text_box = TextBox::new()
        .with_font(font.clone())
        .with_placeholder("9600")
        .fix_width(100.0)
        .lens(models::AppData::baud_rate)
        .padding(5.0)
        .align_left()
        .disabled_if(|data, _| data.connected);

    let connect_button = Button::new("Connect")
        .on_click(|_ctx, data: &mut models::AppData, _env| (*data).connected = true)
        .padding(5.0)
        .center()
        .disabled_if(|data, _| data.connected)
        .background(Color::rgb(0.1, 1.0, 0.2));
    let disconnect_button = Button::new("Disconnect")
        .on_click(|_ctx, data: &mut models::AppData, _env| (*data).connected = false)
        .padding(5.0)
        .center()
        .disabled_if(|data, _| !data.connected)
        .background(Color::rgb(1.0,0.2,0.1));

    let serial_row = Flex::row()
        .with_child(serial_port_label)
        .with_child(serial_port_text_box)
        .with_child(baud_rate_label)
        .with_child(baud_rate_text_box)
        .with_child(connect_button)
        .with_child(disconnect_button);

    Flex::column().with_child(serial_row)
}
