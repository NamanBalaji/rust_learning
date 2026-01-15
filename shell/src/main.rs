use shell::{run, utils::exit};

fn main() {
    match run() {
        Ok(()) => exit(0),
        Err(error) => {
            eprintln!("Error: ${error:?}");
            exit(1);
        }
    }
}
