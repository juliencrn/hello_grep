use std::error::Error;
use std::fs;

pub struct Config {
    pub search: String,
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Arguments missing");
        }
        let search = args[1].clone();
        let path = args[2].clone();

        Ok(Config { search, path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;

    println!("File content:\n{}", content);

    Ok(())
}
