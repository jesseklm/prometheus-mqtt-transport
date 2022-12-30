pub const DEFAULT_CONFIG_FILE: &str = "/etc/prometheus-mqtt-transport/fetch.yaml";
pub const DEFAULT_INTERVAL: i64 = 60;
pub const DEFAULT_SCRAPE_TIMEOUT: u64 = 10;
pub const SCRAPE_NAME_LABEL: &str = "prometheus_mqtt_transport_scrape";

pub fn generate_user_agent() -> String {
    format!(
        "{}/{}",
        global::constants::PACKAGE_NAME,
        global::constants::VERSION
    )
}
