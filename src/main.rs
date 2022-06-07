use std::env;
use std::fs;
use std::io;

struct Config<'a> {
    filename: &'a str,
    query: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Self {
        let filename: &String = &args[1];
        let query: &String = &args[2];
        Self { filename, query }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let Config { filename, query } = Config::new(&args);

    let file_contents: String = read_file(&filename, &query).unwrap();
    println!("{}", file_contents);
}

fn read_file(filename: &str, query: &str) -> Result<String, io::Error> {
    let file_contents: String = fs::read_to_string(filename)?;
    Ok(file_contents)
}
