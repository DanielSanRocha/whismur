use druid::{Data, Lens};
use druid::im::Vector;

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct AppData {
    pub serial_port: String,
    pub baud_rate: String,
    pub rules: Vector<Rule>,
    pub connected: bool
}

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct Rule {
    pub character: char,
    pub channel: u8,
    pub code: u8,
    pub data: u8
}
