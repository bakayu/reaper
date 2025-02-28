use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let config = parse_config(&args);

    // let query = &args[1];
    // let file_path = &args[2];

    println!("Searching for {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Unable to read the file.");

    println!("Text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
