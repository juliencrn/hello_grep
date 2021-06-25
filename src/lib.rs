use ansi_term::Colour::Green;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    pub pattern: String,

    #[structopt(parse(from_os_str))]
    pub path: PathBuf,

    #[structopt(short = "i", help = "Make search case insensitive")]
    pub case_insensitive: bool,
}

#[derive(Debug)]
pub struct Line {
    number: usize,
    content: String,
}

// TODO: Rename in Match and create static method matches
impl Line {
    pub fn new(number: usize, content: String) -> Line {
        Line { number, content }
    }

    pub fn fmt(&self) -> String {
        format!("{}: {}", Green.paint(self.number.to_string()), self.content)
    }
}

pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.pattern, &content)
    } else {
        search_case_sensitive(&config.pattern, &content)
    };

    if results.len() < 1 {
        // TODO: Don't print in code, return Err instead
        println!("There is no result ¯\\(ツ)/¯")
    }

    for line in results {
        println!("{}", line.fmt())
    }

    Ok(())
}

pub fn search_case_sensitive(query: &str, content: &str) -> Vec<Line> {
    let mut results: Vec<Line> = vec![];

    for (index, line) in content.lines().enumerate() {
        if line.contains(query) {
            results.push(Line::new(index, line.to_string()));
        }
    }

    results
}

pub fn search_case_insensitive(query: &str, content: &str) -> Vec<Line> {
    let mut results: Vec<Line> = vec![];
    let query = query.to_lowercase();

    for (index, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push(Line::new(index, line.to_string()));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Cli {
        Cli {
            pattern: String::from("run"),
            path: PathBuf::from("./src/lib.rs"),
            case_insensitive: false,
        }
    }

    #[test]
    fn run_should_not_panic() -> Result<(), String> {
        let config = create_test_config();
        run(config).unwrap();
        Ok(())
    }

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let expected = vec![Line::new(2, "safe, fast, productive.".to_string())];
        let result = search_case_sensitive(pattern, content);
       
        for i in 0..result.len() {
            assert_eq!(expected[i].content, result[i].content)
        }
    }

    #[test]
    fn case_insensitive() {
        let pattern = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expected = vec![
            Line::new(1, "Rust:".to_string()),
            Line::new(4, "Trust me.".to_string())
        ];
        let result = search_case_sensitive(pattern, content);
       
        for i in 0..result.len() {
            assert_eq!(expected[i].content, result[i].content)
        }
    }
}
