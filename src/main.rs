use minigrap::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(e) = minigrap::run(config) {
        process::exit(1);
    }
}
