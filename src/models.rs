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

pub fn parse_rules(rules: Vector<Rule>) -> Result<Vector<ParsedRule>, String> {
    let mut ps_rules = Vector::new();

    for rule in rules {
        let ch = match rule.character.chars().next() {
            Some(c) => c,
            None => return Err(String::from("Rule with empty character!"))
        };

        let channel = match rule.channel.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse() {
            Ok(c) => c,
            Err(_) => return Err(String::from("Channel must be an integer!"))
        };

        let code = match rule.code.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse() {
            Ok(c) => c,
            Err(_) => return Err(String::from("Code must be an integer!"))
        };

        let data = match rule.data.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse() {
            Ok(d) => d,
            Err(_) => return Err(String::from("Data must be an integer!"))
        };

        let velocity = match rule.velocity.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse() {
            Ok(v) => v,
            Err(_) => return Err(String::from("Velocity must be an integer!"))
        };

        ps_rules.push_back(ParsedRule {character: ch, channel, code, data, velocity});
    }

    Ok(ps_rules)
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Status {
    pub connected: bool,
    pub message: String
}

pub struct Midi {
    pub data: u8,
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
}
