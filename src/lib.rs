use std::error::Error;
use std::fs;
use std::io;

pub struct Config<'a> {
    pub filename: &'a str,
    pub query: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed to rustgrep.");
        }
        let filename: &String = &args[1];
        let query: &String = &args[2];
        Ok(Self { filename, query })
    }
}

pub fn read_file(filename: &str, query: &str) -> Result<String, io::Error> {
    let file_contents: String = fs::read_to_string(filename)?;
    Ok(file_contents)
}

pub fn run(filename: &str, query: &str) -> Result<(), Box<dyn Error>> {
    let file_contents: String = read_file(&filename, &query)?;
    println!("{}", file_contents);
    Ok(())
}
