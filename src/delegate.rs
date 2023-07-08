use druid::{Env, AppDelegate, DelegateCtx, Target, Command, commands, Handled};

use crate::models;

pub struct Delegate;

impl AppDelegate<models::AppData> for Delegate {
    fn command(&mut self, _ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut models::AppData, _env: &Env) -> Handled {
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            let json = serde_json::to_string(&data).expect("Error serializing AppState!");
            if let Err(e) = std::fs::write(file_info.path(), json) {
                println!("{}", e.to_string());
                return Handled::No;
            } else {
                return Handled::Yes;
            }
        }

        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(data_s) => {
                    match serde_json::from_str(&data_s) {
                        Ok(new_data) => *data = new_data,
                        Err(e) => {
                            println!("Error decoding json data: {e}");
                            return Handled::No;
                        }
                    };
                    return Handled::Yes;
                }
                Err(e) => {
                    println!("Error opening file: {e}");
                    return Handled::No;
                }
            }
        }

        if let Some(_) = cmd.get(commands::CLOSE_WINDOW) {
            println!("Closing program...");
            std::process::exit(0);
        }

        return Handled::Yes;
    }
}