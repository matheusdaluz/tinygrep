use minigrap::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(e) = minigrap::run(config) {
        process::exit(1);
    }
}
