use crate::cli::Cli;
use crate::line::Line;

fn is_matches(config: &Cli, line: &str) -> bool {
    match config.get_regex().find(line.trim()) {
        Some(x) => {
            if config.line_regexp {
                return x.start() == 0 && x.end() == line.trim().len();
            }
            true
        }
        None => false,
    }
}

pub fn search_all(config: &Cli, content: &str) -> Vec<Line> {
    let mut results: Vec<Line> = vec![];

    for (index, line) in content.lines().enumerate() {
        let matches = if config.invert_match || config.files_without_match {
            !is_matches(config, line)
        } else {
            is_matches(config, line)
        };

        if matches {
            results.push(Line::new(index, line.to_string()));
        }
    }

    results
}

pub fn search_match(config: &Cli, content: &str) -> bool {
    for line in content.lines() {
        let matches = if config.invert_match {
            !is_matches(config, line)
        } else {
            is_matches(config, line)
        };

        if matches {
            return true;
        }
    }

    false
}
