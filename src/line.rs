use crate::cli::CommandLineArgs;
use crate::search;
use crate::utils::colorize;
use ansi_term::Colour::{Green, Red};
use regex::Captures;

#[derive(Debug)]
pub struct Line {
    pub number: usize,
    pub content: String,
}

impl Line {
    pub fn new(number: usize, content: String) -> Line {
        Line { number, content }
    }

    pub fn fmt_line(&self, config: &CommandLineArgs) -> String {
        let regex = search::get_regex(&config.pattern, config.case_insensitive);
        let colorize_pattern =
            |c: &Captures| format!("{}", colorize(Red, &c[0], config.display_color));
        let formatted_line = format!(
            "\t{}",
            regex
                .replace_all(&self.content, colorize_pattern)
                .to_string()
                .trim_end()
        );

        if config.show_line_number {
            let line_number = colorize(Green, &self.number.to_string(), config.display_color);
            format!("{}: {}", line_number, formatted_line)
        } else {
            format!("{}", formatted_line)
        }
    }
}
