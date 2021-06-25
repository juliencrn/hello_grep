use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Arguments missing");
        }
        let query = args[1].clone();
        let path = args[2].clone();
        // TODO: Test if is it 1 or true, not only unset
        // TODO: To be able to pass a CLI argument like -i
        let case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            path,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    if results.len() < 1 {
        println!("There is no result ¯\\(ツ)/¯")
    }

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = vec![];

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = vec![];
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
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
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
