use hello_grep::Cli;
use structopt::StructOpt;
use std::process;

fn main() {
    let config = Cli::from_args();

    if let Err(e) = hello_grep::run(config) {
        eprintln!("Error in execution: {}", e);
        process::exit(1);
    }
}
