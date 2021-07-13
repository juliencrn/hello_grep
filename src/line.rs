use ansi_term::Colour::{Green, Red};
use regex::Captures;

use crate::cli::Cli;

#[derive(Debug)]
pub struct Line {
    pub number: usize,
    pub content: String,
}

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
