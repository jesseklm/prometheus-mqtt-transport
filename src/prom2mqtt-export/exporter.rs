use crate::constants;

use lazy_static::lazy_static;
use log::error;
use prometheus::{Histogram, HistogramOpts, IntCounter, Registry, TextEncoder};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref BYTES_RECEIVED_NO_COMP_TOTAL: IntCounter = IntCounter::new(
        constants::METRICS_BYTES_RECEIVED_NO_COMP_TOTAL_NAME,
        constants::METRICS_BYTES_RECEIVED_NO_COMP_TOTAL_HELP
    )
    .unwrap();
    pub static ref MESSAGES_RECEIVED_NO_COMP_TOTAL: IntCounter = IntCounter::new(
        constants::METRICS_MESSAGES_RECEIVED_NO_COMP_TOTAL_NAME,
        constants::METRICS_MESSAGES_RECEIVED_NO_COMP_TOTAL_HELP
    )
    .unwrap();
    pub static ref BYTES_RECEIVED_COMP_TOTAL: IntCounter = IntCounter::new(
        constants::METRICS_BYTES_RECEIVED_COMP_TOTAL_NAME,
        constants::METRICS_BYTES_RECEIVED_COMP_TOTAL_HELP
    )
    .unwrap();
    pub static ref MESSAGES_RECEIVED_COMP_TOTAL: IntCounter = IntCounter::new(
        constants::METRICS_MESSAGES_RECEIVED_COMP_TOTAL_NAME,
        constants::METRICS_MESSAGES_RECEIVED_COMP_TOTAL_HELP
    )
    .unwrap();
    pub static ref BYTES_RECEIVED_DECOMP_TOTAL: IntCounter = IntCounter::new(
        constants::METRICS_BYTES_RECEIVED_DECOMP_TOTAL_NAME,
        constants::METRICS_BYTES_RECEIVED_DECOMP_TOTAL_HELP
    )
    .unwrap();
    pub static ref DECOMPRESS_TIME: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            constants::METRICS_DECOMPRESS_TIME_NAME,
            constants::METRICS_DECOMPRESS_TIME_HELP
        )
        .buckets(constants::METRICS_DECOMPRESS_BUCKETS.to_vec())
    )
    .unwrap();
    pub static ref PAYLOAD_PARSE_TIME: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            constants::METRICS_PAYLOAD_PARSE_TIME_NAME,
            constants::METRICS_PAYLOAD_PARSE_TIME_HELP
        )
        .buckets(constants::METRICS_PAYLOAD_PARSE_TIME_BUCKETS.to_vec())
    )
    .unwrap();
}

pub fn register() {
    REGISTRY
        .register(Box::new(BYTES_RECEIVED_NO_COMP_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_RECEIVED_NO_COMP_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(BYTES_RECEIVED_COMP_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_RECEIVED_COMP_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(BYTES_RECEIVED_DECOMP_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(DECOMPRESS_TIME.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PAYLOAD_PARSE_TIME.clone()))
        .unwrap();
}

pub fn metrics() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = String::new();

    // Export internal process metrics
    if let Err(e) = encoder.encode_utf8(&prometheus::gather(), &mut buffer) {
        error!("can't export internal process metrics - {}", e);
    }

    buffer
}
