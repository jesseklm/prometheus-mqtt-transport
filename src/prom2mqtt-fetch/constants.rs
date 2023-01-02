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
pub const METRIC_MQTT_QOS_NAME: &str = "prom2mqtt_fetch_mqtt_qos";
pub const METRIC_MQTT_QOS_HELP: &str = "QoS for MQTT messages";
pub const METRIC_MQTT_SEND_NAME: &str = "prom2mqtt_fetch_mqtt_send_seconds";
pub const METRIC_MQTT_SEND_HELP: &str = "Time to send MQTT data";
pub const METRIC_MQTT_SUCCESS_NAME: &str = "prom2mqtt_fetch_mqtt_send_success";
pub const METRIC_MQTT_SUCCESS_HELP: &str = "Success status of MQTT message publishing";

pub const HTML_ROOT: &str = "<html>\n<head><title>Prometheus MQTT transport - scraper</title></head>\n<body>\n<h1>Prometheus MQTT transport - scraper</h1>\n<p><a href=\"/metrics\">Metrics</a></p>\n</body>\n</html>\n";
pub const HTTP_NOT_FOUND: &str = "Not found";
pub const HTTP_METHOD_NOT_ALLOWED: &str = "Method not allowed";
