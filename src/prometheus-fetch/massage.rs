use flate2::read::GzEncoder;
use flate2::Compression;
use log::{debug, info};
use simple_error::bail;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;

pub fn parse_scrape_data(
    raw: &str,
    name: &str,
    expiration: i64,
) -> Result<global::payload::Message, Box<dyn Error>> {
    let mut message = global::payload::Message {
        name: name.to_string(),
        expiration,
        payload: Vec::<global::payload::Payload>::new(),
    };
    let mut metrics: HashMap<&str, global::payload::Payload> = HashMap::new();

    for raw_line in raw.lines() {
        let line = raw_line.trim();

        if line.starts_with('#') {
            let fields: Vec<&str> = line.split_ascii_whitespace().collect();
            // Prometheus HELP/TYPE fields have *at least* four fields
            if fields.len() < 4 {
                bail!(
                    "malformed Prometheus metric line: too few fields in \"{}\"",
                    raw_line
                );
            }

            // Check field type
            let ftype = match fields[1] {
                "HELP" | "TYPE" => fields[1],
                _ => {
                    bail!(
                        "malformed Prometheus metric line: invalid type {}",
                        fields[1]
                    );
                }
            };
            let mname = fields[2];
            let fdata = fields[3..].join(" ");
            let entry = metrics.entry(mname).or_default();
            entry.metric_name = mname.to_string();

            if ftype == "HELP" {
                entry.help = fdata;
            } else {
                entry.data_type = fdata;
            }
        } else {
            let mvec: Vec<&str> = if line.contains('{') {
                line.splitn(2, '{').collect()
            } else {
                line.splitn(2, ' ').collect()
            };
            let mname = mvec[0];
            let entry = metrics.entry(mname).or_default();
            entry.metric_name = mname.to_string();
            entry.data.push(line.to_string());
        }
    }

    for (_, value) in metrics {
        message.payload.push(value);
    }

    Ok(message)
}

fn inject_normalized_name(original: String) -> String {
    if original.contains('{') {
    } else {

    }
}

pub fn build_mqtt_message(
    msg: &Vec<global::payload::Message>,
    compress: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let payload_str = serde_json::to_string(&msg)?;
    let payload: Vec<u8>;
    if compress {
        let cmprs = std::time::Instant::now();

        debug!("compressing data");

        let before = payload_str.len();
        payload = compress_data(payload_str)?;
        let after = payload.len();

        info!(
            "payload data compressed using gzip in {} seconds, {:.2}% saved ({} bytes -> {} bytes)",
            cmprs.elapsed().as_secs_f64(),
            100.0_f64 * (before as f64 - after as f64) / before as f64,
            before,
            after
        );
    } else {
        payload = payload_str.into_bytes();
    }

    Ok(payload)
}

fn compress_data(s: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut gzencoded = GzEncoder::new(s.as_bytes(), Compression::best());
    let mut compressed = Vec::<u8>::new();
    gzencoded.read_to_end(&mut compressed)?;
    Ok(compressed)
}
