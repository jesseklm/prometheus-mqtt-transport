use crate::config;

use log::{debug, error, info, warn};
use std::error::Error;
use std::sync::mpsc;

pub fn run(
    cfg: &config::Configuration,
    receiver: mpsc::Receiver<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    debug!("creating MQTT connection");
    let mqtt_conn_opts = global::mqtt::connection_builder(&cfg.mqtt)?;

    debug!("creating MQTT client");
    let mqtt_client = global::mqtt::client_builder(&cfg.mqtt)?;

    info!("connecting to MQTT broker {}", cfg.mqtt.broker);
    mqtt_client.connect(mqtt_conn_opts)?;

    loop {
        let data = receiver.recv()?;
        // XXX: Shouldn't happen but just to make sure
        if data.len() < 2 {
            error!("received data is too short ({} bytes)", data.len());
            continue;
        }

        if !mqtt_client.is_connected() {
            warn!(
                "connection to MQTT broker {} lost, reconnecting",
                cfg.mqtt.broker
            );
            if let Err(e) = mqtt_client.reconnect() {
                error!(
                    "reconnection to MQTT broker {} failed - {}",
                    cfg.mqtt.broker, e
                );
                continue;
            }
        }

        info!(
            "sending {} bytes of data to topic {} on {}",
            data.len(),
            &cfg.mqtt.topic,
            &cfg.mqtt.broker
        );
        let msg = paho_mqtt::message::Message::new(&cfg.mqtt.topic, data, cfg.mqtt.qos);
        if let Err(e) = mqtt_client.publish(msg) {
            error!("sending message to MQTT broker failed - {}", e);
            continue;
        }
    }
}
