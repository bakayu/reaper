use std::env;
use std::process;

use reaper::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = reaper::run(config) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
