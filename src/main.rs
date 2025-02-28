use std::env;
use std::process;

use reaper::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);

    if let Err(e) = reaper::run(config) {
        println!("error: {e}");
        process::exit(1);
    }
}
