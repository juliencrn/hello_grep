use hello_grep::cli::CommandLineArgs;
use std::io::stdout;
use std::process;
use structopt::StructOpt;

fn main() {
    let config = CommandLineArgs::from_args();

    if let Err(e) = hello_grep::run(config, &mut stdout()) {
        eprintln!("Error in execution: {}", e);
        process::exit(1);
    }
}
