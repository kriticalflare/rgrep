use rgrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rgrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
