use crate::constants;

use lazy_static::lazy_static;
use log::error;
use prometheus::{Gauge, GaugeVec, IntGauge, IntGaugeVec, Opts, Registry, TextEncoder};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref SCRAPE_DURATION: GaugeVec = GaugeVec::new(
        Opts::new(
            constants::METRIC_SCRAPE_DURATION_NAME,
            constants::METRIC_SCRAPE_DURATION_HELP
        ),
        &["scrape_name"],
    )
    .unwrap();
    pub static ref SCRAPE_SUCCESS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_SCRAPE_SUCCESS_NAME,
            constants::METRIC_SCRAPE_SUCCESS_HELP
        ),
        &["scrape_name"],
    )
    .unwrap();
    pub static ref COMPRESSION: IntGauge = IntGauge::new(
        constants::METRIC_COMPRESSION_NAME,
        constants::METRIC_COMPRESSION_HELP
    )
    .unwrap();
    pub static ref SIZE: IntGauge = IntGauge::new(
        constants::METRIC_METRICS_SIZE_NAME,
        constants::METRIC_METRICS_SIZE_HELP
    )
    .unwrap();
    pub static ref COMPRESSED_SIZE: IntGauge = IntGauge::new(
        constants::METRIC_COMPRESSED_SIZE_NAME,
        constants::METRIC_COMPRESSED_SIZE_HELP
    )
    .unwrap();
    pub static ref COMPRESS_TIME: Gauge = Gauge::new(
        constants::METRIC_COMPRESS_TIME_NAME,
        constants::METRIC_COMPRESS_TIME_HELP
    )
    .unwrap();
    pub static ref MQTT_QOS: IntGauge = IntGauge::new(
        constants::METRIC_MQTT_QOS_NAME,
        constants::METRIC_MQTT_QOS_HELP
    )
    .unwrap();
    pub static ref MQTT_SEND: Gauge = Gauge::new(
        constants::METRIC_MQTT_SEND_NAME,
        constants::METRIC_MQTT_SEND_HELP
    )
    .unwrap();
    pub static ref MQTT_SUCCESS: IntGauge = IntGauge::new(
        constants::METRIC_MQTT_SUCCESS_NAME,
        constants::METRIC_MQTT_SUCCESS_HELP
    )
    .unwrap();
}

pub fn register() {
    REGISTRY
        .register(Box::new(SCRAPE_DURATION.clone()))
        .unwrap();
    REGISTRY.register(Box::new(SCRAPE_SUCCESS.clone())).unwrap();
    REGISTRY.register(Box::new(COMPRESSION.clone())).unwrap();
    REGISTRY.register(Box::new(SIZE.clone())).unwrap();
    REGISTRY
        .register(Box::new(COMPRESSED_SIZE.clone()))
        .unwrap();
    REGISTRY.register(Box::new(COMPRESS_TIME.clone())).unwrap();
    REGISTRY.register(Box::new(MQTT_QOS.clone())).unwrap();
    REGISTRY.register(Box::new(MQTT_SEND.clone())).unwrap();
    REGISTRY.register(Box::new(MQTT_SUCCESS.clone())).unwrap();
}

pub fn metrics() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = String::new();

    // Export scrape metrics
    if let Err(e) = encoder.encode_utf8(&REGISTRY.gather(), &mut buffer) {
        error!("can't export scrape metrics - {}", e);
    }

    // Export internal process metrics
    if let Err(e) = encoder.encode_utf8(&prometheus::gather(), &mut buffer) {
        error!("can't export internal process metrics - {}", e);
    }

    buffer
}
