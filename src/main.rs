use druid::widget::{Scroll, Flex, Label, TextBox, Button, Align, List};
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc, FontDescriptor, FontFamily, Color};
use druid::im::Vector;
use models::AppData;

mod models;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .title("Whismur")
        .with_min_size((950.0,420.0))
        .window_size((950.0,420.0));

    let data = models::AppData {
        serial_port: String::from("/dev/ttyACM0"),
        baud_rate: String::from("9600"),
        rules: Vector::new(),
        connected: false
    };

    AppLauncher::with_window(main_window).log_to_console().launch(data)
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

    let rules_label: Align<models::AppData> = Label::new("Rules")
        .with_font(font.clone())
        .padding(10.0)
        .fix_height(30.0)
        .center();

    let small_font = FontDescriptor::new(FontFamily::MONOSPACE)
        .with_size(20.0);

    let rules_scroll = Scroll::new(List::new(move || {
            Flex::row()
                .with_child(Label::new("Character").with_font(small_font.clone()))
                .with_child(TextBox::new()
                    .with_font(small_font.clone())
                    .lens(models::Rule::character)
                )
                .with_child(Label::new("Channel").with_font(small_font.clone()))
                .with_child(TextBox::new()
                    .with_font(small_font.clone())
                    .lens(models::Rule::channel)
                )
                .with_child(Label::new("Code").with_font(small_font.clone()))
                .with_child(TextBox::new()
                    .with_font(small_font.clone())
                    .lens(models::Rule::code)
                )
                .with_child(Label::new("Data").with_font(small_font.clone()))
                .with_child(TextBox::new()
                    .with_font(small_font.clone())
                    .lens(models::Rule::data)
                )
        }).background(Color::rgb(0.4,0.4,0.4))
        .lens(models::AppData::rules))
        .fix_height(250.0)
        .padding(20.0);

    let add_rule_button: Align<models::AppData> = Button::new("Add Rule")
        .on_click(|_ctx, data: &mut AppData, _env| {
            (*data).rules.push_back(models::Rule {character: "a".to_string(), channel: "0".to_string(), code: "0".to_string(), data: "0".to_string()})
        })
        .padding(5.0)
        .fix_width(100.0)
        .center();
    let remove_rule_button = Button::new("Remove Last Rule")
        .on_click(|_ctx, data: &mut AppData, _env| {
            let l = (*data).rules.len();
            if l > 0 {
                (*data).rules.remove(l - 1);
            }
        })
        .disabled_if(|data, _| (*data).rules.len() == 0)
        .padding(5.0);
    let save_button: Align<models::AppData> = Button::new("Save")
        .padding(5.0)
        .fix_width(100.0)
        .center();
    let load_button: Align<models::AppData> = Button::new("Load")
        .padding(5.0)
        .fix_width(100.0)
        .center();

    let footer_label: Align<models::AppData> = Label::new("Status: Disconnected!")
        .with_font(font.clone())
        .with_text_color(Color::rgb(1.0,0.2,0.2))
        .padding(5.0)
        .align_right();
    let footer_row = Flex::row()
       .with_child(remove_rule_button)
       .with_child(add_rule_button)
       .with_child(save_button)
       .with_child(load_button)
       .with_child(footer_label)
       .align_right();

    Flex::column()
        .with_child(serial_row)
        .with_child(rules_label)
        .with_child(rules_scroll)
        .with_child(footer_row)
}
