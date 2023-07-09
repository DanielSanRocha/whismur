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
    pub data: String,
    pub velocity: String
}

#[derive(Clone)]
pub struct ParsedRule {
    pub character: char,
    pub channel: u8,
    pub code: u8,
    pub data: u8,
    pub velocity: u8
}

pub fn parse_rules(rules: Vector<Rule>) -> Vector<ParsedRule> {
    let mut ps_rules = Vector::new();

    for rule in rules {
        let ch = rule.character.chars().next().expect("Empty Rule!");
        let channel = rule.channel.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().expect("Channel must be an integer!");
        let code = rule.code.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().expect("Channel must be an integer!");
        let data = rule.data.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().expect("Channel must be an integer!");
        let velocity = rule.velocity.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().expect("Channel must be an integer!");

        ps_rules.push_back(ParsedRule {character: ch, channel: channel, code: code, data: data, velocity: velocity});
    }

    ps_rules
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Status {
    pub connected: bool,
    pub message: String
}

pub struct MIDI {
    pub data: u8,
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
}
