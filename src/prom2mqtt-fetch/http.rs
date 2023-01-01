use crate::constants;

use log::debug;
use simple_error::bail;
use std::error::Error;
use std::time;

pub fn build_http_client(timeout: u64) -> Result<reqwest::blocking::Client, Box<dyn Error>> {
    let dtimeout = time::Duration::from_secs(timeout);
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        "X-Clacks-Overhead",
        reqwest::header::HeaderValue::from_static("GNU Terry Pratchett"),
    );

    let http_client_builder = reqwest::blocking::ClientBuilder::new()
        .user_agent(constants::generate_user_agent())
        .default_headers(headers)
        .timeout(dtimeout);
    let http_client = match http_client_builder.build() {
        Ok(v) => v,
        Err(e) => bail!("can't create HTTP client: {}", e),
    };
    Ok(http_client)
}

pub fn get(client: &reqwest::blocking::Client, url: &str) -> Result<String, Box<dyn Error>> {
    debug!("sending HTTP GET request to {}", url);
    let reply = client.get(url).send()?;
    if reply.status() != reqwest::StatusCode::OK {
        bail!(
            "HTTP request to {} returned {} instead of \"200 OK\"",
            url,
            reply.status()
        );
    }
    Ok(reply.text()?)
}
