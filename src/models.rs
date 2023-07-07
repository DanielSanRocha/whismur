use druid::{Data, Lens};

#[derive(Debug, Clone, Data, Lens)]
pub struct AppData {
    pub serial_port: String,
    pub baud_rate: String,
    pub connected: bool
}
