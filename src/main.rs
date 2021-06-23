use mini_grep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem while interpreting arguments: {}", err);
        process::exit(1);
    });

    println!("Search query: {}", config.search);
    println!("Path to file: {}", config.path);

    if let Err(e) = mini_grep::run(config) {
        println!("Error in execution: {}", e);
        process::exit(1);
    }
}
