use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_slice: &[String] = &args[1..];
    println!("{:#?}", args_slice);

    // Alternately we can hard code the address/reference of the arguments into variables
    let file_name: &String = &args[1];
    let regex_to_search: &String = &args[2];

    println!("Searching for {} in file: {}", regex_to_search, file_name);
}
