use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    #[serde(default)]
    pub expiration: i64,
    pub name: String,
    pub payload: Vec<Payload>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub data: Vec<String>,
    #[serde(alias = "type")]
    pub data_type: String,
    pub help: String,
    pub metric_name: String,
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

impl Message {
    pub fn new() -> Self {
        Message {
            expiration: 0,
            name: String::new(),
            payload: Vec::<Payload>::new(),
        }
    }
}

impl Default for Payload {
    fn default() -> Self {
        Self::new()
    }
}

impl Payload {
    pub fn new() -> Self {
        Payload {
            data: Vec::<String>::new(),
            data_type: String::new(),
            help: String::new(),
            metric_name: String::new(),
        }
    }
}
