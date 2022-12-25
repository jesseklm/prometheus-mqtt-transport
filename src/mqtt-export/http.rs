use crate::config;
use crate::constants;
use crate::data;

use log::error;
use simple_error::bail;
use std::error::Error;
use std::sync::mpsc;

pub fn run(
    cfg: &config::Configuration,
    data_request: mpsc::Sender<data::Data>,
    data_reply: mpsc::Receiver<String>,
) -> Result<(), Box<dyn Error>> {
    let headers: Vec<tiny_http::Header> = vec![
        tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap(),
        tiny_http::Header::from_bytes(&b"X-Clacks-Overhead"[..], &b"GNU Terry Pratchett"[..])
            .unwrap(),
    ];

    let server = match tiny_http::Server::http(&cfg.prometheus.listen) {
        Ok(v) => v,
        Err(e) => bail!("cant start HTTP server - {}", e),
    };

    let mpath = &cfg.prometheus.path;

    loop {
        let request = match server.recv() {
            Ok(v) => v,
            Err(e) => {
                error!("can't process incoming HTTP  request: {}", e);
                continue;
            }
        };
        let method = request.method();
        let url = request.url();
        let status_code: tiny_http::StatusCode;
        let payload: String;
        let http_header = headers.clone();

        if method == &tiny_http::Method::Get {
            match url {
                "/" => {
                    status_code = tiny_http::StatusCode::from(302_i16);
                    payload = constants::HTML_ROOT.to_string();
                }
                mpath => {
                    // TODO: Send request to data_channel and wait for reply
                    let reply = String::new();
                    status_code = tiny_http::StatusCode::from(200_i16);
                    payload = reply;
                }
                _ => {
                    status_code = tiny_http::StatusCode::from(404_i16);
                    payload = constants::HTTP_NOT_FOUND.to_string();
                }
            };
        } else {
            status_code = tiny_http::StatusCode::from(405_i16);
            payload = constants::HTTP_METHOD_NOT_ALLOWED.to_string();
        }

        if let Err(e) = request.respond(tiny_http::Response::new(
            status_code,
            http_header,
            payload.as_bytes(),
            Some(payload.len()),
            None,
        )) {
            error!("can't send response back to client - {}", e);
        }
    }
}
