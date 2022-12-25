use crate::constants;
use log::debug;
use serde::Deserialize;
use simple_error::bail;
use std::error::Error;
use std::fs;
use url::Url;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    #[serde(default)]
    pub prometheus: Prometheus,
    pub mqtt: global::mqtt::MQTT,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Prometheus {
    #[serde(default = "prometheus_default_listen")]
    pub listen: String,
    #[serde(default = "prometheus_default_path")]
    pub path: String,
}

impl Default for Prometheus {
    fn default() -> Self {
        Prometheus {
            listen: constants::DEFAULT_LISTEN_ADDR.to_string(),
            path: constants::DEFAULT_METRICS_PATH.to_string(),
        }
    }
}

fn prometheus_default_listen() -> String {
    constants::DEFAULT_LISTEN_ADDR.to_string()
}

fn prometheus_default_path() -> String {
    constants::DEFAULT_METRICS_PATH.to_string()
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let raw = fs::read_to_string(f)?;
    let parsed: Configuration = serde_yaml::from_str(raw.as_str())?;

    validate(&parsed)?;

    debug!("parsed configuration: {:?}", parsed);

    Ok(parsed)
}

fn validate(cfg: &Configuration) -> Result<(), Box<dyn Error>> {
    if let Err(e) = validate_url(&cfg.mqtt.broker) {
        bail!("invalid broker URL - {}", e);
    }
    if cfg.mqtt.qos > 2 || cfg.mqtt.qos < 0 {
        bail!("invalid MQTT QoS setting");
    }
    if cfg.mqtt.topic.is_empty() || cfg.mqtt.topic.contains('+') || cfg.mqtt.topic.contains('#') {
        bail!("invalid MQTT topic")
    }
    if cfg.mqtt.timeout == 0 {
        bail!("invalid MQTT timeout");
    }
    if cfg.mqtt.reconnect_timeout == 0 {
        bail!("invalid MQTT reconnect timeout");
    }

    if cfg.prometheus.listen.is_empty() {
        bail!("invlid listener address");
    }
    if cfg.prometheus.path.is_empty() {
        bail!("invalid metrics path");
    }

    Ok(())
}

fn validate_url(s: &str) -> Result<(), Box<dyn Error>> {
    let _parsed = Url::parse(s)?;
    Ok(())
}
