use crate::constants;
use log::debug;
use serde::Deserialize;
use simple_error::bail;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use url::Url;

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
    #[serde(skip)]
    pub http_client: Option<reqwest::blocking::Client>,
    pub interval: Option<i64>,
    #[serde(skip)]
    pub last_scrape: i64,
    pub name: String,
    pub timeout: Option<u64>,
    pub url: String,
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
    let mut parsed: Configuration = serde_yaml::from_str(raw.as_str())?;

    validate(&parsed)?;

    parsed.mqtt.topic = parsed.mqtt.topic.trim_end_matches('/').to_string();
    parsed.mqtt.topic = format!(
        "{}/{}",
        parsed.mqtt.topic,
        gethostname::gethostname().into_string().unwrap()
    );

    debug!("parsed configuration: {:?}", parsed);

    Ok(parsed)
}

fn validate(cfg: &Configuration) -> Result<(), Box<dyn Error>> {
    let mut names: HashSet<String> = HashSet::new();

    if cfg.global.interval <= 0 {
        bail!("invalid interval value in global section");
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

    for s in cfg.scrape.iter() {
        if s.name.is_empty() {
            bail!("no name set for scrape job");
        }
        if names.contains(&s.name.clone()) {
            bail!("duplicate scrape name '{}'", s.name);
        }
        names.insert(s.name.clone());

        if s.url.is_empty() {
            bail!("no URL to scrape found");
        }
        if let Err(e) = validate_url(&s.url) {
            bail!("invalid URL for scrape '{}' - {}", s.name, e);
        }

        if let Some(v) = s.interval {
            if v <= 0 {
                bail!("invalid interval value for scrape interval in {}", s.name);
            }
        }
    }

    Ok(())
}

fn validate_url(s: &str) -> Result<(), Box<dyn Error>> {
    let _parsed = Url::parse(s)?;
    Ok(())
}
