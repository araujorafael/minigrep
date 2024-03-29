use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Search for {:?}", config.query);
    println!("In file {:?}", config.filename);

    // Does not need use 'unwrap_or_else' becouse
    // run function does not return a concrete value to unwrap
    // we just need the side effects of run :)
    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
