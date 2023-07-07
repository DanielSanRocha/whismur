use druid::widget::{Flex, Label, TextBox, Button};
use druid::{LocalizedString, AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc, FontDescriptor, FontFamily};
use models::AppData;

mod models;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .with_min_size((900.0,400.0))
        .title("Whismur")
        .window_size((900.0,400.0));

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

    let connect_button_string = LocalizedString::new("hello-counter")
        .with_arg("count", |data: &AppData, _env| data.baud_rate.clone().into());

    let connect_button = Button::new(connect_button_string)
        .on_click(|_ctx, data: &mut models::AppData, _env| (*data).connected = !data.connected)
        .padding(5.0)
        .center();

    let serial_row = Flex::row()
        .with_child(serial_port_label)
        .with_child(serial_port_text_box)
        .with_child(baud_rate_label)
        .with_child(baud_rate_text_box)
        .with_child(connect_button);

    Flex::column().with_child(serial_row)
}
