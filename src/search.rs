use crate::cli::Cli;
use crate::line::Line;

fn is_matches(config: &Cli, line: &str) -> bool {
    match config.get_regex().find(line.trim()) {
        Some(x) => {
            if config.invert_match {
                return false;
            }

            if !config.line_regexp {
                return true;
            }

            x.start() == 0 && x.end() == line.trim().len()
        }
        None => config.invert_match,
    }
}

pub fn search(config: &Cli, content: &str) -> Vec<Line> {
    let mut results: Vec<Line> = vec![];

    for (index, line) in content.lines().enumerate() {
        if is_matches(config, line) {
            results.push(Line::new(index, line.to_string()));
        }
    }

    results
}
