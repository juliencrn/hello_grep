use hello_grep::cli::Cli;
use std::process;
use structopt::StructOpt;

fn main() {
    let config = Cli::from_args();

    if let Err(e) = hello_grep::run(config) {
        eprintln!("Error in execution: {}", e);
        process::exit(1);
    }
}
