use std::{env, process};
use minigrep;

fn main() {
    let config = minigrep::Config::parse_configs(env::args()).unwrap_or_else(|err| {
        eprintln!("Error en la aplicacion: {}",err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error en la aplicacion: {}", e);
        process::exit(1);
    }
}