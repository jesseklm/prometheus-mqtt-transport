use crate::config;
use crate::data;

use log::{debug, error, info, warn};
use simple_error::bail;
use std::error::Error;
use std::sync::mpsc;

pub fn run(
    cfg: &config::Configuration,
    data_sender: mpsc::Sender<data::Data>,
) -> Result<(), Box<dyn Error>> {
    let conn = global::mqtt::connection_builder(&cfg.mqtt)?;
    let client = global::mqtt::client_builder(&cfg.mqtt)?;

    let cstatus = client.connect(conn)?;
    if let Some(v) = cstatus.connect_response() {
        if !v.session_present {
            info!(
                "subscribing to topic {} on {} qith QoS {}",
                cfg.mqtt.topic, cfg.mqtt.broker, cfg.mqtt.qos
            );
            if let Err(e) = client.subscribe(&cfg.mqtt.topic, cfg.mqtt.qos) {
                bail!("can't subscribe to topic {} - {}", cfg.mqtt.topic, e);
            }
        }
    } else {
        bail!("empty connect_response result from MQTT connection");
    };

    let messages = client.start_consuming();

    for msg in messages.iter() {
        match msg {
            Some(vmsg) => {
                debug!("data received on topic");
                let pdata = match data::parse_raw_metrics(vmsg.payload().to_vec()) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("can't parse received data - {}", e);
                        continue;
                    }
                };
                info!("received {} bytes of data on {}", pdata.len(), vmsg.topic());
                debug!("sending parsed data to data handler");
                data_sender.send(data::Data::MetricData(pdata))?;
            }
            None => {
                if !client.is_connected() {
                    warn!("connection to broker was lost, reconnecting");
                    client.reconnect()?;
                }
            }
        }
    }

    Ok(())
}
