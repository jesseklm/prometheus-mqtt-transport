use flate2::bufread::GzDecoder;
use log::{debug, error};
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
    // HELP and TYPE strings *must* occur only once!
    let mut metrics_type: HashMap<String, String> = HashMap::new();
    let mut metrics_help: HashMap<String, String> = HashMap::new();
    let mut metrics_data: HashMap<String, Vec<String>> = HashMap::new();

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
                                    metrics_type.insert(
                                        mtrc.metric_name.to_string(),
                                        mtrc.data_type.clone(),
                                    );
                                    metrics_help
                                        .insert(mtrc.metric_name.to_string(), mtrc.help.clone());
                                    let collected_data = metrics_data
                                        .entry(mtrc.metric_name.to_string())
                                        .or_default();
                                    collected_data.extend(mtrc.data.clone());
                                    debug!(
                                        "collected type ({}), help ({}) and data ({}) for '{}'",
                                        mtrc.data_type.clone(),
                                        mtrc.help.clone(),
                                        mtrc.data.clone().len(),
                                        mtrc.metric_name
                                    );
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
                for m in metrics_type.keys() {
                    if let Some(mtype) = metrics_type.get(m) {
                        if let Some(mhelp) = metrics_help.get(m) {
                            if let Some(mdata) = metrics_data.get(m) {
                                debug!(
                                    "'{}' - TYPE: {} / HELP: {} / len(data) = {}",
                                    m,
                                    mtype,
                                    mhelp,
                                    mdata.len()
                                );

                                // TODO: for _count data for histograms, the data *must* folloow
                                // immediately after the histogram and summary data (type: histogram or summary). Otherwise
                                // promtool check metrics will complain
                                let type_str = if !mtype.is_empty() {
                                    format!("# TYPE {} {}", m, mtype)
                                } else {
                                    String::new()
                                };
                                let help_str = if !mhelp.is_empty() {
                                    format!("# HELP {} {}", m, mhelp)
                                } else {
                                    String::new()
                                };
                                reply.push(format!(
                                    "{}\n{}\n{}\n",
                                    type_str,
                                    mdata.join("\n"),
                                    help_str
                                ));
                            }
                        } else {
                            error!("key {} found in metrics_type map but not in metrics_help, invalid metric format?", m);
                        }
                    }
                }
                http_reply.send(reply.join("\n"))?;

                metrics_type.clear();
                metrics_help.clear();
                metrics_data.clear();

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
