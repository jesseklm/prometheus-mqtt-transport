pub const DEFAULT_CONFIG_FILE: &str = "/etc/prometheus-mqtt-transport/export.yaml";
pub const DEFAULT_LISTEN_ADDR: &str = "localhost:9991";
pub const DEFAULT_METRICS_PATH: &str = "/metrics";
pub const HTML_ROOT: &str = "<html>\n<head><title>Prometheus MQTT transport</title></head>\n<body>\n<h1>Prometheus MQTT transport</h1>\n<p><a href=\"/metrics\">Metrics</a></p>\n</body>\n</html>\n";
pub const HTTP_NOT_FOUND: &str = "Not found";
pub const HTTP_METHOD_NOT_ALLOWED: &str = "Method not allowed";
