pub const DEFAULT_CONFIG_FILE: &str = "/etc/prometheus-mqtt-transport/export.yaml";
pub const DEFAULT_LISTEN_ADDR: &str = "localhost:9991";
pub const DEFAULT_METRICS_PATH: &str = "/metrics";
pub const HTML_ROOT: &str = "<html>\n<head><title>Prometheus MQTT transport</title></head>\n<body>\n<h1>Prometheus MQTT transport</h1>\n<p><a href=\"/metrics\">Metrics</a></p>\n</body>\n</html>\n";
pub const HTTP_NOT_FOUND: &str = "Not found";
pub const HTTP_METHOD_NOT_ALLOWED: &str = "Method not allowed";

pub const METRICS_BYTES_RECEIVED_NO_COMP_TOTAL_HELP: &str = "Bytes of uncompressed metric received";
pub const METRICS_BYTES_RECEIVED_NO_COMP_TOTAL_NAME: &str =
    "prom2mqtt_export_uncompressed_bytes_received_total";
pub const METRICS_MESSAGES_RECEIVED_NO_COMP_TOTAL_NAME: &str =
    "prom2mqtt_export_uncompressed_messages_received_total";
pub const METRICS_MESSAGES_RECEIVED_NO_COMP_TOTAL_HELP: &str =
    "Uncompressed metric messages received";

pub const METRICS_BYTES_RECEIVED_COMP_TOTAL_HELP: &str = "Bytes of compressed metric received";
pub const METRICS_BYTES_RECEIVED_COMP_TOTAL_NAME: &str =
    "prom2mqtt_export_compressed_bytes_received_total";
pub const METRICS_MESSAGES_RECEIVED_COMP_TOTAL_NAME: &str =
    "prom2mqtt_export_compressed_messages_received_total";
pub const METRICS_MESSAGES_RECEIVED_COMP_TOTAL_HELP: &str = "Uncompressed metric messages received";
pub const METRICS_BYTES_RECEIVED_DECOMP_TOTAL_HELP: &str =
    "Decompressed size of compressed metric received";
pub const METRICS_BYTES_RECEIVED_DECOMP_TOTAL_NAME: &str =
    "prom2mqtt_export_decompressed_bytes_received_total";

pub const METRICS_DECOMPRESS_TIME_NAME: &str = "prom2mqtt_export_decompress_seconds";
pub const METRICS_DECOMPRESS_TIME_HELP: &str = "Time to decompress compressed metric data";
pub const METRICS_DECOMPRESS_BUCKETS: &[f64; 19] = &[
    0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8,
    0.9, 1.0,
];

pub const METRICS_PAYLOAD_PARSE_TIME_NAME: &str = "prom2mqtt_export_payload_parse_seconds";
pub const METRICS_PAYLOAD_PARSE_TIME_HELP: &str = "Time to parse received payload";
pub const METRICS_PAYLOAD_PARSE_TIME_BUCKETS: &[f64; 23] = &[
    0.01, 0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8,
    0.85, 0.9, 0.95, 1.0, 1.5, 2.0,
];
