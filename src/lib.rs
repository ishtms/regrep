use std::env;
use std::error::Error;
use std::fs;
use std::io;

pub const CASE_INSENSITIVE: &str = "CASE_INSENSITIVE";

pub struct Config<'a> {
    pub filename: &'a str,
    pub query: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed to rustgrep.");
        }
        let filename: &String = &args[1];
        let query: &String = &args[2];

        let case_sensitive = env::var(CASE_INSENSITIVE).is_err();

        Ok(Self {
            filename,
            query,
            case_sensitive,
        })
    }
}

pub fn read_file(filename: &str) -> Result<String, io::Error> {
    let file_contents: String = fs::read_to_string(filename)?;
    Ok(file_contents)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let Config {
        query,
        filename,
        case_sensitive,
    } = config;

    let file_contents: String = read_file(&filename)?;
    let matched_entries: Option<Vec<(&str, usize)>>;

    if case_sensitive {
        matched_entries = search(query, &file_contents);
    } else {
        matched_entries = search_case_insensitive(query, &file_contents);
    }

    if let Some(matches) = matched_entries {
        for curr_line in matches.iter() {
            println!("Line {}: {}", curr_line.1, curr_line.0);
        }
        println!("\nTotal {} matches found!", matches.len());

        Ok(())
    } else {
        println!("No match found.");
        Ok(())
    }
}

fn search<'a>(query: &'a str, contents: &'a str) -> Option<Vec<(&'a str, usize)>> {
    let mut result_vector = Vec::new();
    for (index, curr_line) in contents.lines().enumerate() {
        if curr_line.contains(query) {
            result_vector.push((curr_line, index + 1));
        }
    }
    if result_vector.len() > 0 {
        Some(result_vector)
    } else {
        None
    }
}

fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Option<Vec<(&'a str, usize)>> {
    let mut result_vector = Vec::new();

    for (line_number, line_content) in contents.lines().enumerate() {
        if line_content.to_lowercase().contains(&query.to_lowercase()) {
            result_vector.push((line_content, line_number + 1));
        }
    }
    if result_vector.len() > 0 {
        Some(result_vector)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let contents = "\
fn main() {
    doSomething();
}
main();";
        let query = "doS";
        let result = search(query, contents);
        assert_eq!(result, Some(vec![("    doSomething();", 2)]));

        let query_to_fail = "dos";
        let result = search(query_to_fail, contents);
        assert_eq!(result, None);
    }
    #[test]
    fn case_insensitive() {
        let contents = "\
fn main() {
    doSomething();
}";
        let query = "dos";

        let result = search_case_insensitive(query, contents);
        assert_eq!(result, Some(vec![("    doSomething();", 2)]));

        let query = "dOs";
        let result = search_case_insensitive(query, contents);
        assert_eq!(result, Some(vec![("    doSomething();", 2)]));
    }
}
