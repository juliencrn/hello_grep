use ansi_term::Colour::Cyan;
use std::error::Error;
use std::fs;

pub mod cli;
pub mod line;
pub mod search;

use self::cli::Cli;

pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {
    if config.path.len() == 0 {
        // TODO: Should throw an error and stop the program
        println!("No files found");
    }

    let mut file_count: usize = 0;
    let mut match_count: usize = 0;
    let mut printed_count: usize = 0;
    let paths = &config.path.to_vec();
    let hide_filename = config.no_filename || paths.len() == 1;

    for path in paths {
        let pathname = path.clone();
        let pathname = pathname.to_str().unwrap();
        let content = fs::read_to_string(path)?;
        let results = search::search(&config, &content);

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
                // Display matches (title then lines)
                if printed_count < config.max && !hide_filename {
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

    if match_count == 0 {
        // TODO: Should throw an error and stop the program
        println!("There is no result ¯\\(ツ)/¯");
    } else if config.stats {
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use self::cli::Cli;
    use self::line::Line;
    use super::*;
    use std::path::PathBuf;

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
            no_filename: false,
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
        let result = search::search(&config, content);

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
        let result = search::search(&config, content);

        for i in 0..result.len() {
            assert_eq!(expected[i].content, result[i].content)
        }
    }
}
