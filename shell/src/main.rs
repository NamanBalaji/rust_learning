use std::process::exit;

use shell::run;

fn main() {
    match run() {
        Ok(()) => exit(0),
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    }
}
