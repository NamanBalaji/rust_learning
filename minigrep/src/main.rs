use std::{env, process};

use minigrep::Config;

fn main() {
    let config: Config;
    let config_result = Config::build(env::args());
    match config_result {
        Ok(c) => {
            config = c;
        }
        Err(e) => {
            eprintln!("Problem parsing arguments: {e}");
            process::exit(1);
        }
    }

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

