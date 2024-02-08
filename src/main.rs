use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error occurred while parsing arguments: {err}");
        process::exit(1);
    });

    println!("Seraching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application failed: {e}");
        process::exit(1);
    }
}
