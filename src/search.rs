use crate::line::Line;
use regex::{Regex, RegexBuilder};

/// Build a regex from a pattern and the case insensitive flag.
pub fn get_regex(pattern: &str, case_insensitive: bool) -> Regex {
    RegexBuilder::new(&pattern)
        .case_insensitive(case_insensitive)
        .build()
        .expect("Invalid Regex")
}

/// For a given line, test if the regex matches.
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

/// Search all matches in a text and return an array of lines.
pub fn get_all_matches<'a>(
    regex: &Regex,
    content: &'a str,
    invert_match: bool,
    line_regexp: bool,
) -> Vec<Line<'a>> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| is_matches(regex, line, invert_match, line_regexp))
        .map(|(index, line)| Line::new(index, &line))
        .collect()
}

/// Return true at the first match or false instead.
pub fn has_match(regex: &Regex, content: &str, invert_match: bool, line_regexp: bool) -> bool {
    for line in content.lines() {
        if is_matches(regex, line, invert_match, line_regexp) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> &'static str {
        "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
Duct tape."
    }

    #[test]
    fn test_is_matches() {
        let content = "safe, fast, productive.";
        let regex = get_regex("safe", false);
        let result = is_matches(&regex, content, false, false);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_matches_inverted_1() {
        let content = "safe, fast, productive.";
        let regex = get_regex("safe", false);
        let result = is_matches(&regex, content, true, false);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_matches_inverted_2() {
        let content = "safe, fast, productive.";
        let regex = get_regex("rabbit", false);
        let result = is_matches(&regex, content, true, false);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_matches_line_regexp_1() {
        let content = "safe, fast, productive.";
        let regex = get_regex("safe", false);
        let result = is_matches(&regex, content, false, true);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_matches_line_regexp_2() {
        let content = "safe, fast, productive.";
        let regex = get_regex("safe, fast, productive.", false);
        let result = is_matches(&regex, content, false, true);
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_match_without_match() {
        let regex = get_regex("bike", true);
        let content = get_test_content();
        let result = has_match(&regex, content, false, false);
        assert_eq!(result, false);
    }

    #[test]
    fn test_search_match_with_match() {
        let regex = get_regex("safe", true);
        let content = get_test_content();
        let result = has_match(&regex, content, false, false);
        assert_eq!(result, true);
    }

    #[test]
    fn search_all_case_sensitive() {
        let regex = get_regex("duck", false);
        let content = get_test_content();
        let expected = vec![Line::new(2, "safe, fast, productive.")];
        let result = get_all_matches(&regex, content, false, false);

        for i in 0..result.len() {
            assert_eq!(expected[i].content, result[i].content)
        }
    }

    #[test]
    fn search_all_case_insensitive() {
        let regex = get_regex("rUsT", true);
        let content = get_test_content();
        let expected = vec![Line::new(1, "Rust:"), Line::new(4, "Trust me.")];
        let result = get_all_matches(&regex, content, false, false);

        for i in 0..result.len() {
            assert_eq!(expected[i].content, result[i].content)
        }
    }

    #[test]
    fn search_all_inverted() {
        let regex = get_regex("rUsT", true);
        let content = get_test_content();
        let expected = vec![
            Line::new(2, "safe, fast, productive."),
            Line::new(3, "Pick three."),
            Line::new(5, "Duct tape."),
        ];
        let result = get_all_matches(&regex, content, true, false);

        for i in 0..expected.len() {
            assert_eq!(expected[i].content, result[i].content)
        }
    }
}
