use druid::{Data, Lens};
use druid::im::Vector;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens, Default)]
pub struct AppData {
    pub serial_port: String,
    pub baud_rate: String,
    pub rules: Vector<Rule>,
    #[serde(skip_serializing, skip_deserializing)]
    pub connected: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens, Default)]
pub struct Rule {
    pub character: String,
    pub channel: String,
    pub code: String,
    pub data: String
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Status {
    pub connected: bool,
    pub message: String
}
