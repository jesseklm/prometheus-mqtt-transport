use flate2::bufread::GzDecoder;
use log::debug;
use simple_error::bail;
use std::error::Error;
use std::io::prelude::*;
use std::string::String;
use std::sync::mpsc;

pub enum Data {
    HTTPRequest,
    MetricData(Vec<global::payload::Message>),
}

pub fn handler(
    data_receiver: mpsc::Receiver<Data>,
    http_reply: mpsc::Sender<String>,
) -> Result<(), Box<dyn Error>> {
    loop {
        let request = data_receiver.recv()?;
        match request {
            Data::HTTPRequest => {
                debug!("HTTP request received");
            }
            Data::MetricData(msg) => {
                debug!("{} metric messages received", msg.len());
            }
        };
    }
}

pub fn parse_raw_metrics(raw: Vec<u8>) -> Result<Vec<global::payload::Message>, Box<dyn Error>> {
    if raw.len() < 2 {
        bail!("received payload is too short");
    }

    let data_str = if raw[0] == 0x1f && raw[1] == 0x8b {
        let mut gzd = GzDecoder::new(&raw[..]);
        let mut decompressed = String::new();
        gzd.read_to_string(&mut decompressed)?;
        decompressed
    } else {
        String::from_utf8(raw)?
    };

    let parsed = serde_json::from_str(&data_str)?;
    Ok(parsed)
}
