use ansi_term::Colour;
use ansi_term::Colour::{Cyan, Green, Red};
use regex::{Captures, Regex, RegexBuilder};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    pub pattern: String,

    #[structopt(parse(from_os_str))]
    pub path: Vec<PathBuf>,

    #[structopt(
        short = "i",
        long = "ignore-case",
        help = "Make search case insensitive"
    )]
    pub case_insensitive: bool,

    #[structopt(short = "n", long = "line-number", help = "Show line number")]
    pub show_line_number: bool,

    #[structopt(long = "color", help = "Activate color in output")]
    pub display_color: bool,

    #[structopt(
        short = "v",
        long = "invert-match",
        help = "Invert the sense of matching"
    )]
    pub invert_match: bool,

    #[structopt(
        short = "c",
        long = "count",
        help = "Suppress normal output; instead print a count of matching lines for each input file"
    )]
    pub count: bool,

    #[structopt(
        short = "s",
        long = "stats",
        help = "Display match statistics at the end"
    )]
    pub stats: bool,

    #[structopt(
        short = "x",
        long = "line-regexp",
        help = "Select only those matches that exactly match the whole line"
    )]
    pub line_regexp: bool,

    #[structopt(
        short = "m",
        long = "max-count",
        help = "Stop reading a file after NUM matching lines",
        default_value = "1000"
    )]
    pub max: usize,
}

impl Cli {
    fn get_regex(&self) -> Regex {
        RegexBuilder::new(&self.pattern)
            .case_insensitive(self.case_insensitive)
            .build()
            .expect("Invalid Regex")
    }

    fn is_matches(&self, line: &str) -> bool {
        match self.get_regex().find(line.trim()) {
            Some(x) => {
                if self.invert_match {
                    return false;
                }

                if !self.line_regexp {
                    return true;
                }

                x.start() == 0 && x.end() == line.trim().len()
            }
            None => self.invert_match,
        }
    }

    fn colorize(&self, color: Colour, text: &str) -> String {
        if self.display_color {
            format!("{}", color.paint(text))
        } else {
            text.to_string()
        }
    }

    pub fn search(&self, content: &str) -> Vec<Line> {
        let mut results: Vec<Line> = vec![];

        for (index, line) in content.lines().enumerate() {
            if self.is_matches(line) {
                results.push(Line::new(index, line.to_string()));
            }
        }

        results
    }
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

    pub fn fmt_line(&self, config: &Cli) -> String {
        let regex = config.get_regex();
        let colorize_pattern = |c: &Captures| format!("{}", config.colorize(Red, &c[0]));
        let formatted_line = format!(
            "\t{}",
            regex
                .replace_all(&self.content, colorize_pattern)
                .to_string()
                .trim_end()
        );

        if config.show_line_number {
            let line_number = config.colorize(Green, &self.number.to_string());
            format!("{}: {}", line_number, formatted_line)
        } else {
            format!("{}", formatted_line)
        }
    }
}

pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {
    let mut file_count: usize = 0;
    let mut match_count: usize = 0;
    let mut printed_count: usize = 0;

    if config.path.len() == 0 {
        // TODO: Should throw an error and stop the program
        println!("No files found");
    }

    let paths = &config.path.to_vec();

    for path in paths {
        let pathname = path.clone();
        let pathname = pathname.to_str().unwrap();
        let content = fs::read_to_string(path)?;
        let results = config.search(&content);

        if results.len() > 0 {
            file_count += 1;
            match_count = match_count + results.len();

            if config.count {
                // Display only filenames
                if printed_count < config.max {
                    println!("{}: \t{}", pathname, results.len());
                    printed_count += 1;
                }
            } else {
                // Display matches
                if printed_count < config.max {
                    println!("\n{}", config.colorize(Cyan, pathname));
                }

                for line in results {
                    if printed_count < config.max {
                        println!("{}", line.fmt_line(&config));
                        printed_count += 1;
                    }
                }
            }
        }
    }

    if match_count > 0 {
        if config.stats {
            if printed_count != match_count {
                println!(
                    "\n{} match(es) found (including {} hidden) in {} file(s).",
                    match_count,
                    match_count - printed_count,
                    file_count
                );
            } else {
                println!(
                    "\n{} match(es) found in {} file(s).",
                    match_count, file_count
                );
            }
        }
    } else {
        println!("There is no result ¯\\(ツ)/¯")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config(pattern: &str, case_insensitive: bool) -> Cli {
        Cli {
            pattern: String::from(pattern),
            path: vec![PathBuf::from("./src/lib.rs")],
            case_insensitive,
            show_line_number: true,
            display_color: false,
            stats: false,
            count: false,
            invert_match: false,
            line_regexp: false,
            max: 1000,
        }
    }

    #[test]
    fn run_should_not_panic() -> Result<(), String> {
        let config = create_test_config("run", true);
        run(config).unwrap();
        Ok(())
    }

    #[test]
    fn case_sensitive() {
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let config = create_test_config("duct", false);
        let expected = vec![Line::new(2, "safe, fast, productive.".to_string())];
        let result = config.search(content);

        for i in 0..result.len() {
            assert_eq!(expected[i].content, result[i].content)
        }
    }

    #[test]
    fn case_insensitive() {
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let config = create_test_config("rUsT", true);
        let expected = vec![
            Line::new(1, "Rust:".to_string()),
            Line::new(4, "Trust me.".to_string()),
        ];
        let result = config.search(content);

        for i in 0..result.len() {
            assert_eq!(expected[i].content, result[i].content)
        }
    }
}
