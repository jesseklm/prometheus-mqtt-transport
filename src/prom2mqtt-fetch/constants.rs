pub const DEFAULT_CONFIG_FILE: &str = "/etc/prometheus-mqtt-transport/fetch.yaml";
pub const DEFAULT_INTERVAL: i64 = 60;
pub const DEFAULT_SCRAPE_TIMEOUT: u64 = 10;
pub const SCRAPE_NAME_LABEL: &str = "prom2mqtt_fetch_scrape";
pub const DEFAULT_PROMETHEUS_LISTEN: &str = "localhost:9998";
pub const DEFAULT_PROMETHEUS_PATH: &str = "/metrics";

pub fn generate_user_agent() -> String {
    format!(
        "{}/{}",
        global::constants::PACKAGE_NAME,
        global::constants::VERSION
    )
}

// Metrics for each defined scrape job
pub const METRIC_SCRAPE_DURATION_NAME: &str = "prom2mqtt_fetch_scrape_duration_seconds";
pub const METRIC_SCRAPE_DURATION_HELP: &str = "Duration of metric scrape";
pub const METRIC_SCRAPE_DURATION_BUCKETS: &[f64; 46] = &[
    0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35, 0.40,
    0.45, 0.50, 0.55, 0.60, 0.65, 0.70, 0.75, 0.80, 0.85, 0.90, 0.95, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5,
    4.0, 4.5, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5, 8.0, 8.5, 9.0, 9.5, 10.0,
];

pub const METRIC_SCRAPE_SUCCESS_NAME: &str = "prom2mqtt_fetch_scrape_success";
pub const METRIC_SCRAPE_SUCCESS_HELP: &str = "Success status of scrape";

// Metrics for processing of scraped data
pub const METRIC_COMPRESSION_NAME: &str = "prom2mqtt_fetch_compression";
pub const METRIC_COMPRESSION_HELP: &str = "Whether gzip compression of scraped data is enabled";

pub const METRIC_METRICS_SIZE_NAME: &str = "prom2mqtt_fetch_metrics_bytes";
pub const METRIC_METRICS_SIZE_HELP: &str = "Size of all scraped metrics";

pub const METRIC_COMPRESSED_SIZE_NAME: &str = "prom2mqtt_fetch_compressed_metrics_bytes";
pub const METRIC_COMPRESSED_SIZE_HELP: &str = "Compressed size of all metrics";

pub const METRIC_COMPRESS_TIME_NAME: &str = "prom2mqtt_fetch_compress_seconds";
pub const METRIC_COMPRESS_TIME_HELP: &str = "Time to compress metric data";
pub const METRIC_COMPRESS_TIME_BUCKETS: &[f64; 42] = &[
    0.025, 0.05, 0.075, 0.1, 0.125, 0.15, 0.175, 0.2, 0.225, 0.25, 0.275, 0.3, 0.325, 0.35, 0.375,
    0.4, 0.425, 0.45, 0.475, 0.5, 0.525, 0.55, 0.575, 0.6, 0.625, 0.65, 0.675, 0.7, 0.725, 0.75,
    0.775, 0.8, 0.825, 0.85, 0.875, 0.9, 0.925, 0.95, 0.975, 1.0, 1.5, 2.0,
];

pub const METRIC_MQTT_QOS_NAME: &str = "prom2mqtt_fetch_mqtt_qos";
pub const METRIC_MQTT_QOS_HELP: &str = "QoS for MQTT messages";

pub const METRIC_MQTT_SEND_TIME_NAME: &str = "prom2mqtt_fetch_mqtt_send_seconds";
pub const METRIC_MQTT_SEND_TIME_HELP: &str = "Time to send MQTT data";
pub const METRIC_MQTT_SEND_TIME_BUCKETS: &[f64; 21] = &[
    0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8,
    0.9, 1.0, 2.5, 5.0,
];

pub const METRIC_MQTT_SUCCESS_NAME: &str = "prom2mqtt_fetch_mqtt_send_success";
pub const METRIC_MQTT_SUCCESS_HELP: &str = "Success status of MQTT message publishing";

pub const HTML_ROOT: &str = "<html>\n<head><title>Prometheus MQTT transport - scraper</title></head>\n<body>\n<h1>Prometheus MQTT transport - scraper</h1>\n<p><a href=\"/metrics\">Metrics</a></p>\n</body>\n</html>\n";
pub const HTTP_NOT_FOUND: &str = "Not found";
pub const HTTP_METHOD_NOT_ALLOWED: &str = "Method not allowed";
