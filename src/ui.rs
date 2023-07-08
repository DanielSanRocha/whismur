use std::sync::mpsc::{Sender, Receiver};
use druid::{commands, FileSpec, Key, Widget, WidgetExt, FontDescriptor, FontFamily, Color, FileDialogOptions};
use druid::widget::{Scroll, Flex, Label, TextBox, Button, Align, List};

use crate::models;

pub fn ui_builder(tx_data: Sender<models::AppData>, rx_status: Receiver<models::Status>, tx_disconnect: Sender<bool>, rx_status2: Receiver<models::Status>) -> impl Widget<models::AppData> {
    let json = FileSpec::new("JSON File", &["json"]);

    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![json])
        .default_type(json)
        .default_name(String::from("whismur.json"))
        .title("Save the current program state")
        .button_text("Save");
    let load_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![json])
        .default_type(json)
        .default_name(String::from("whismur.json"))
        .title("Load the program state")
        .button_text("Load");

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
        .on_click(move |_ctx, data: &mut models::AppData, _env| {
            let _ = tx_data.send((*data).clone());
            let status =  rx_status.recv().expect("Error receiving status from thread!");
            if status.connected {
                (*data).connected = true;
            } else {
                println!("{}", status.message);
            }
        })
        .padding(5.0)
        .center()
        .disabled_if(|data, _| data.connected)
        .background(Color::rgb(0.1, 1.0, 0.2));
    let disconnect_button = Button::new("Disconnect")
        .on_click(move |_ctx, data: &mut models::AppData, _env| {
            let _ = tx_disconnect.send(true);
            let status = rx_status2.recv().expect("Erro receiving disconnect status from thread!");
            if status.connected == false {
                (*data).connected = false;
            } else {
                println!("{}", status.message);
            }
        })
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
                .with_child(Label::new("Velocity").with_font(small_font.clone()))
                .with_child(TextBox::new()
                    .with_font(small_font.clone())
                    .lens(models::Rule::velocity))
        }).background(Color::rgb(0.4,0.4,0.4))
        .lens(models::AppData::rules))
        .fix_height(250.0)
        .padding(20.0)
        .disabled_if(|data, _| (*data).connected);

    let add_rule_button: Align<models::AppData> = Button::new("Add Rule")
        .on_click(|_ctx, data: &mut models::AppData, _env| {
            (*data).rules.push_back(models::Rule {character: "a".to_string(), channel: "0".to_string(), code: "0".to_string(), data: "0".to_string(), velocity: "0".to_string()})
        })
        .disabled_if(|data, _| (*data).connected)
        .padding(5.0)
        .fix_width(100.0)
        .center();
    let remove_rule_button = Button::new("Remove Last Rule")
        .on_click(|_ctx, data: &mut models::AppData, _env| {
            let l = (*data).rules.len();
            if l > 0 {
                (*data).rules.remove(l - 1);
            }
        })
        .disabled_if(|data, _| (*data).rules.len() == 0 || (*data).connected)
        .padding(5.0);
    let save_button: Align<models::AppData> = Button::new("Save")
        .on_click(move |ctx, _: &mut models::AppData, _| {
            ctx.submit_command(commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()))
        })
        .disabled_if(|data, _| (*data).connected)
        .padding(5.0)
        .fix_width(100.0)
        .center();
    let load_button: Align<models::AppData> = Button::new("Load")
        .on_click(move |ctx, _: &mut models::AppData, _| {
            ctx.submit_command(commands::SHOW_OPEN_PANEL.with(load_dialog_options.clone()))
        })
        .disabled_if(|data, _| (*data).connected)
        .padding(5.0)
        .fix_width(100.0)
        .center();

    let footer_label_color_key = Key::new("color");
    let footer_label = Label::new(
        |data: &models::AppData, _env: &_| {
            if data.connected {String::from("     Status: Connected")}
            else {String::from("  Status: Disconnected")}
        })
        .with_font(font.clone())
        .with_text_color(footer_label_color_key.clone())
        .padding(5.0)
        .align_right()
        .env_scope(move |env, data| {
            if data.connected {
                env.set(footer_label_color_key.clone(), Color::rgb(0.2,1.0,0.2))
            } else {
                env.set(footer_label_color_key.clone(), Color::rgb(1.0,1.0,0.2))
            }
        });
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
