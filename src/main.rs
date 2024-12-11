use std::{env, process};
use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::parse_configs(&args).unwrap_or_else(|err| {
        eprintln!("Error en la aplicacion: {}",err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error en la aplicacion: {}", e);
        process::exit(1);
    }
}