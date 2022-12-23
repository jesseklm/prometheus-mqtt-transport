use crate::constants;
use serde::Deserialize;
use simple_error::bail;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    #[serde(default)]
    pub global: Global,
    pub mqtt: global::mqtt::MQTT,
    pub scrape: Vec<Scrape>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Global {
    #[serde(default = "default_global_interval")]
    pub interval: i64,
    #[serde(default)]
    pub compress: bool,
    #[serde(default = "default_global_timeout")]
    pub timeout: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Scrape {
    pub name: String,
    pub url: String,
    pub compress: Option<bool>,
    pub interval: Option<i64>,
    pub timeout: Option<u64>,
    #[serde(skip)]
    pub last_scrape: i64,
    #[serde(skip)]
    pub http_client: Option<reqwest::blocking::Client>,
}

impl Default for Global {
    fn default() -> Self {
        Global {
            interval: constants::DEFAULT_INTERVAL,
            compress: false,
            timeout: constants::DEFAULT_SCRAPE_TIMEOUT,
        }
    }
}

fn default_global_interval() -> i64 {
    constants::DEFAULT_INTERVAL
}

fn default_global_timeout() -> u64 {
    constants::DEFAULT_SCRAPE_TIMEOUT
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let raw = fs::read_to_string(f)?;
    let parsed: Configuration = serde_yaml::from_str(raw.as_str())?;

    validate(&parsed)?;
    Ok(parsed)
}

fn validate(cfg: &Configuration) -> Result<(), Box<dyn Error>> {
    if cfg.global.interval <= 0 {
        bail!("invalid interval value in global section");
    }

    if cfg.mqtt.qos > 2 {
        bail!("invalid MQTT QoS setting");
    }

    if cfg.mqtt.topic.is_empty() || cfg.mqtt.topic.contains("+") || cfg.mqtt.topic.contains("#") {
        bail!("invalid MQTT topic")
    }

    if cfg.mqtt.timeout == 0 {
        bail!("invalid MQTT timeout");
    }

    if cfg.mqtt.reconnect_timeout == 0 {
        bail!("invalid MQTT reconnect timeout");
    }

    for s in cfg.scrape.iter() {
        if s.name.is_empty() {
            bail!("no name set for scrape job");
        }
        if s.url.is_empty() {
            bail!("no URL to scrape found");
        }
        if let Some(v) = s.interval {
            if v <= 0 {
                bail!("invalid interval value for scrape interval in {}", s.name);
            }
        }
    }

    Ok(())
}
