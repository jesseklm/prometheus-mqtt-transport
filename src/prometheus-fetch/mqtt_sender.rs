use crate::config;

use std::error::Error;
use std::sync::mpsc;

pub fn run(
    cfg: &config::Configuration,
    receiver: mpsc::Receiver<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    loop {
        let _data = receiver.recv()?;
    }
}
