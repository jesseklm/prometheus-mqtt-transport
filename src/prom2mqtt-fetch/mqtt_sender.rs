use crate::config;
use crate::exporter;

use log::{debug, error, info, warn};
use std::error::Error;
use std::sync::mpsc;
use std::{thread, time};

pub fn run(
    cfg: &config::Configuration,
    receiver: mpsc::Receiver<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    let one_second = time::Duration::from_secs(1);

    debug!("creating MQTT connection");
    let mqtt_conn_opts = global::mqtt::connection_builder(&cfg.mqtt)?;

    debug!("creating MQTT client");
    let mqtt_client = global::mqtt::client_builder(&cfg.mqtt)?;

    info!("connecting to MQTT broker {}", cfg.mqtt.broker);
    let mut ticktock: u64 = 0;
    loop {
        let mco = mqtt_conn_opts.clone();
        if let Err(e) = mqtt_client.connect(mco) {
            error!(
                "connection to MQTT broker {} failed: {}",
                cfg.mqtt.broker, e
            );
            if ticktock > cfg.mqtt.reconnect_timeout {
                return Err(Box::new(e));
            }
            thread::sleep(one_second);
            ticktock += 1;
            warn!(
                "retryingo to connect to MQTT broker {} - attempt {}/{}",
                cfg.mqtt.broker, ticktock, cfg.mqtt.reconnect_timeout
            );
        } else {
            info!("connected to MQTT broker {}", cfg.mqtt.broker);
            break;
        }
    }

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

        let pubt = std::time::Instant::now();
        info!(
            "sending {} bytes of data to topic {} on {}",
            data.len(),
            &cfg.mqtt.topic,
            &cfg.mqtt.broker
        );
        let msg = paho_mqtt::message::Message::new(&cfg.mqtt.topic, data, cfg.mqtt.qos);
        if let Err(e) = mqtt_client.publish(msg) {
            error!("sending message to MQTT broker failed - {}", e);
            exporter::MQTT_SUCCESS.set(0);
            continue;
        }
        let pubt_elapsed = pubt.elapsed().as_secs_f64();
        exporter::MQTT_SEND_TIME.observe(pubt_elapsed);
        exporter::MQTT_QOS.set(cfg.mqtt.qos as i64);
        exporter::MQTT_SUCCESS.set(1);

        info!("MQTT message send in {} seconds", pubt_elapsed,);
    }
}
