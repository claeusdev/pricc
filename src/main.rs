use std::{env, process};

use pric::{init, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("There was a problem initializing project: {}", err);
        process::exit(1)
    });
    if let Err(err) = init(config) {
        eprintln!("There was a problem initializing project: {}", err);
        process::exit(1)
    }
}
