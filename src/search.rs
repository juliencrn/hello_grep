use crate::line::Line;
use regex::{Regex, RegexBuilder};

pub fn get_regex(pattern: &str, case_insensitive: bool) -> Regex {
    RegexBuilder::new(&pattern)
        .case_insensitive(case_insensitive)
        .build()
        .expect("Invalid Regex")
}

fn is_matches(regex: &Regex, line: &str, reversed: bool, line_regexp: bool) -> bool {
    match regex.find(line.trim()) {
        Some(x) => {
            if reversed {
                return false;
            }
            if line_regexp {
                // Compare the len of the result with the original line.len().
                return x.start() == 0 && x.end() == line.trim().len();
            }
            true
        }
        None => reversed,
    }
}

pub fn search_all(
    regex: &Regex,
    content: &str,
    invert_match: bool,
    line_regexp: bool,
) -> Vec<Line> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| is_matches(regex, line, invert_match, line_regexp))
        .map(|(index, line)| Line::new(index, line.to_string()))
        .collect()
}

pub fn search_match(regex: &Regex, content: &str, invert_match: bool, line_regexp: bool) -> bool {
    for line in content.lines() {
        if is_matches(regex, line, invert_match, line_regexp) {
            return true;
        }
    }

    false
}
