use flate2::bufread::GzDecoder;
use log::debug;
use simple_error::bail;
use std::collections::HashMap;
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
    let mut metrics: HashMap<String, global::payload::Message> = HashMap::new();
    let mut metrics_expiration: HashMap<String, i64> = HashMap::new();
    let mut now: i64;
    let mut expired: Vec<String> = Vec::new();

    loop {
        let request = data_receiver.recv()?;
        now = chrono::Local::now().timestamp();
        match request {
            Data::HTTPRequest => {
                debug!("HTTP request received");
                let mut reply: Vec<String> = Vec::new();

                debug!("building reply string");
                for name in metrics.keys() {
                    if let Some(mdata) = metrics.get(name) {
                        if let Some(last_update) = metrics_expiration.get(name) {
                            if now - last_update >= mdata.expiration {
                                debug!("'{}' was last updated {} - {} seconds ago, expiration set to {}, adding to removal list", name, last_update, now - last_update, mdata.expiration);
                                expired.push(name.to_string());
                            } else {
                                for mtrc in mdata.payload.iter() {
                                    reply.push(metric_to_string(mtrc));
                                }
                            }
                        } else {
                            // XXX: Should never happen
                            panic!(
                                "BUG: key {} found in metrics map but not in metrics_expiration",
                                name
                            );
                        }
                    }
                }
                http_reply.send(reply.join("\n"))?;

                // remove expired data
                for exp in expired.iter() {
                    debug!("removing expired data for {} from HashMaps", exp);
                    metrics.remove(exp);
                    metrics_expiration.remove(exp);
                }
                expired.clear();
            }
            Data::MetricData(msg) => {
                debug!("{} metric messages received", msg.len());
                for m in msg {
                    let mname = m.name.clone();
                    metrics.insert(m.name.clone(), m);
                    metrics_expiration.insert(mname, now);
                }
            }
        };
    }
}

pub fn metric_to_string(payload: &global::payload::Payload) -> String {
    format!(
        "# TYPE {} {}\n{}\n# HELP {} {}",
        payload.metric_name,
        payload.data_type,
        payload.data.join("\n"),
        payload.metric_name,
        payload.help,
    )
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
