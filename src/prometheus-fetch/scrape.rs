use crate::config;
use crate::http;
use crate::massage;

use log::{debug, error, info};
use std::error::Error;
use std::sync::mpsc;
use std::{thread, time};

pub fn run(
    cfg: &mut config::Configuration,
    sender: mpsc::Sender<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    let one_second = time::Duration::from_secs(1);
    let mut now: i64;

    loop {
        now = chrono::Local::now().timestamp();
        let mut data: Vec<global::payload::Message> = Vec::new();

        // Iterate of scrape list
        for scrape in cfg.scrape.iter_mut() {
            // Even the scrape interval has not passed, create a HTTP client if it does not exist
            let timeout = match scrape.timeout {
                Some(v) => v,
                None => cfg.global.timeout,
            };
            if scrape.http_client.is_none() {
                scrape.http_client = Some(http::build_http_client(timeout)?);
            }

            // check if the interval has been reached
            let interval = match scrape.interval {
                Some(v) => v,
                None => cfg.global.interval,
            };

            if (now - scrape.last_scrape) >= interval {
                let scrp = std::time::Instant::now();

                debug!(
                    "{} - {} == {}, interval is {} -> start scraping {}",
                    now,
                    scrape.last_scrape,
                    now - scrape.last_scrape,
                    interval,
                    scrape.name
                );
                // scrape data
                info!("'{}': scraping data from {}", scrape.name, scrape.url);

                let cli = match &scrape.http_client {
                    Some(v) => v,
                    None => {
                        panic!("Uninitialized HTTP client for scrape {}", scrape.name);
                    }
                };
                let raw = match http::get(cli, &scrape.url) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("scraping of {} failed: {}", scrape.url, e);
                        continue;
                    }
                };
                info!(
                    "'{}': scraped {} bytes from {} for {} in {} seconds",
                    scrape.name,
                    raw.len(),
                    scrape.url,
                    scrape.name,
                    scrp.elapsed().as_secs_f64()
                );

                // Massage raw Prometheus data into MQTT payload
                let parsed =
                    massage::parse_scrape_data(&raw, &scrape.name, &scrape.labels, interval)?;
                data.push(parsed);

                debug!("updating scrape.last_scrape stamp to {}", now);
                scrape.last_scrape = now;
            } else {
                debug!(
                    "{} - {} == {}, interval is {} -> scraping of {} not yet required",
                    now,
                    scrape.last_scrape,
                    now - scrape.last_scrape,
                    interval,
                    scrape.name
                );
            }
        }
        if !data.is_empty() {
            // send to MQTT thread
            debug!("sending data to MQTT thread");
            let mqtt_msg = massage::build_mqtt_message(&data, cfg.global.compress)?;
            sender.send(mqtt_msg)?;
        };
        thread::sleep(one_second);
    }
}
