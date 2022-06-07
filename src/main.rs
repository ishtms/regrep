use std::env;
use std::process;

use rustgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Config { filename, query } = Config::new(&args).unwrap_or_else(|err| {
        println!("There was an error while parsing the arguments: {}", err);
        process::exit(1);
    });

    if let Err(error) = rustgrep::run(filename, query) {
        println!(
            "Something went wrong while parsing the contents of the file: {}",
            error
        );
        process::exit(1);
    }
}
