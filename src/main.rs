use std::env::args;
use std::process::exit;
use wcr::parse_config;

fn main() {
    if let Err(e) = parse_config(args().collect()).and_then(wcr::run) {
        eprintln!("{}", e);
        exit(1);
    }
}
