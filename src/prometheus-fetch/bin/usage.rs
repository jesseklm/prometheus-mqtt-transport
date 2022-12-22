use crate::constants;

pub fn show_usage() {
    global::show_version();
    println!("Usage: {} [-c <cfg>|--config=<cfg>] [-h|--help] [-V|--version]

    -c <cfg>        Read configuration from file <cfg>
    --config=<cfg>  Default: {}

    -h              Shows this text
    --help

    -V              Show version information
    --version

", env!("CARGO_BIN_NAME"), constants::DEFAULT_CONFIG_FILE);
}

