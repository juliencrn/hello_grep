use ansi_term::Colour::{Cyan, Green, Red};
use regex::{Captures, RegexBuilder};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    pub pattern: String,

    #[structopt(parse(from_os_str))]
    pub path: Vec<PathBuf>,

    #[structopt(short = "i", help = "Make search case insensitive")]
    pub case_insensitive: bool,

    #[structopt(short = "n", help = "Show line number")]
    pub num: bool,
}

#[derive(Debug)]
pub struct Line {
    number: usize,
    content: String,
}

// TODO: Rename in Match and create static method "is (-i) matches -> bool"
impl Line {
    pub fn new(number: usize, content: String) -> Line {
        Line { number, content }
    }

    // TODO: There are too many params, better to just pass Cli object
    pub fn fmt(&self, pattern: &str, case_insensitive: bool, show_num: bool) -> String {
        let mut formatted_line = colorize_match(&self.content, pattern, case_insensitive);
        formatted_line = format!("\t{}", formatted_line.trim_end());

        if show_num {
            let line_number = Green.paint(self.number.to_string());
            format!("{}: {}", line_number, &formatted_line)
        } else {
            format!("{}", &formatted_line,)
        }
    }
}

pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {
    let mut file_count: usize = 0;
    let mut match_count: usize = 0;

    if config.path.len() == 0 {
        // TODO: Should throw an error and stop the program
        println!("No files found");
    }

    for path in config.path {
        let pathname = path.clone();
        let pathname = pathname.to_str().unwrap();
        let content = fs::read_to_string(path)?;
        let results = if config.case_insensitive {
            search_case_insensitive(&config.pattern, &content)
        } else {
            search_case_sensitive(&config.pattern, &content)
        };

        if results.len() > 0 {
            println!("\n{}", Cyan.paint(pathname));
            file_count += 1;
            match_count = match_count + results.len();
        }

        for line in results {
            let pretty_line = line.fmt(&config.pattern, config.case_insensitive, config.num);
            println!("{}", pretty_line)
        }
    }

    if match_count > 0 {
        println!(
            "\n{} match(es) found in {} file(s).",
            match_count, file_count
        );
    } else {
        println!("There is no result ¯\\(ツ)/¯")
    }

    Ok(())
}

pub fn colorize_match(line: &str, pattern: &str, case_insensitive: bool) -> String {
    let regex = RegexBuilder::new(&pattern)
        .case_insensitive(case_insensitive)
        .build()
        .expect("Invalid Regex");

    let colorize_pattern = |caps: &Captures| format!("{}", Red.paint(&caps[0]));

    regex.replace_all(line, colorize_pattern).to_string()
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
            path: vec![PathBuf::from("./src/lib.rs")],
            case_insensitive: false,
            num: true,
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
            Line::new(4, "Trust me.".to_string()),
        ];
        let result = search_case_sensitive(pattern, content);

        for i in 0..result.len() {
            assert_eq!(expected[i].content, result[i].content)
        }
    }
}
