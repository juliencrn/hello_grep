use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Arguments missing");
        }
        let query = args[1].clone();
        let path = args[2].clone();

        Ok(Config { query, path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;

    for line in search(&config.query, &content) {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = vec![];

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        let args: Vec<String> = vec![
            String::from("path/to/bin"),
            String::from("run"),
            String::from("./src/lib.rs"),
        ];

        Config::new(&args).unwrap()
    }

    #[test]
    fn create_config() {
        let config = create_test_config();

        assert_eq!(config.query, "run");
        assert_eq!(config.path, "./src/lib.rs");
    }

    #[test]
    #[should_panic]
    fn create_config_without_args() {
        let args: Vec<String> = vec![];
        Config::new(&args).unwrap();
    }

    #[test]
    fn run_should_not_panic() -> Result<(), String> {
        let config = create_test_config();
        run(config).unwrap();
        Ok(())
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
