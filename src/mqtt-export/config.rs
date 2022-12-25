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
}

impl Default for Prometheus {
    fn default() -> Self {
        Prometheus{
            listen: constants::DEFAULT_LISTEN_ADDR,
            path: constants::DEFAULT_METRICS_PATH,
        }
    }
}

