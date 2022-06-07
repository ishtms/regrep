use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::process;

struct Config<'a> {
    filename: &'a str,
    query: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed to rustgrep.");
        }
        let filename: &String = &args[1];
        let query: &String = &args[2];
        Ok(Self { filename, query })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let Config { filename, query } = Config::new(&args).unwrap_or_else(|err| {
        println!("There was an error while parsing the arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching inside file: {}\nQuery to search: {}",
        filename, query
    );
    if let Err(error) = run(filename, query) {
        println!(
            "Something went wrong while parsing the contents of the file: {}",
            error
        );
        process::exit(1);
    }
}

fn read_file(filename: &str, query: &str) -> Result<String, io::Error> {
    let file_contents: String = fs::read_to_string(filename)?;
    Ok(file_contents)
}

fn run(filename: &str, query: &str) -> Result<(), Box<dyn Error>> {
    let file_contents: String = read_file(&filename, &query)?;
    println!("{}", file_contents);
    Ok(())
}
