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
    pub character: String,
    pub channel: String,
    pub code: String,
    pub data: String
}
