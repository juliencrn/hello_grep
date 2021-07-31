use ansi_term::Colour::Cyan;
use std::error::Error;
use std::fs;
use std::io::Write;

pub mod cli;
pub mod line;
pub mod search;
pub mod utils;

use self::cli::CommandLineArgs;
use utils::colorize;

pub fn run(config: CommandLineArgs, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
    if config.path.len() == 0 {
        // TODO: Should throw an error and stop the program
        panic!("No files found");
    }

    let regex = search::get_regex(&config.pattern, config.case_insensitive);
    let paths = &config.path.to_vec();
    let hide_filename = config.no_filename || paths.len() == 1;
    let mut file_count: usize = 0;
    let mut match_count: usize = 0;
    let mut printed_count: usize = 0;

    for path in paths {
        let content = fs::read_to_string(&path)?;
        let pathname = path.to_str().unwrap();

        // Print files instead of matched lines.
        if config.files_with_matches || config.files_without_match {
            if search::has_match(
                &regex,
                &content,
                config.files_without_match,
                config.line_regexp,
            ) {
                writeln!(writer, "{}", &pathname)?;
                printed_count += 1;
            }
        }
        // Print matches
        else {
            let results =
                search::get_all_matches(&regex, &content, config.invert_match, config.line_regexp);

            if results.len() > 0 {
                file_count += 1;
                match_count = match_count + results.len();

                if config.count {
                    // Display only filenames
                    if printed_count < config.max {
                        writeln!(writer, "{}: \t{}", pathname, results.len())?;
                        printed_count += 1;
                    }
                } else {
                    // Display matches (title then lines)
                    if printed_count < config.max && !hide_filename {
                        writeln!(
                            writer,
                            "\n{}",
                            colorize(Cyan, pathname, config.display_color)
                        )?;
                    }

                    for line in results {
                        if printed_count < config.max {
                            writeln!(writer, "{}", line.fmt_line(&config))?;
                            printed_count += 1;
                        }
                    }
                }
            }
        }
    }

    if printed_count == 0 {
        // TODO: Should throw an error and stop the program
        writeln!(writer, "There is no result ¯\\(ツ)/¯")?;
    } else if config.stats && (!config.files_with_matches || !config.files_without_match) {
        if printed_count != match_count {
            writeln!(
                writer,
                "\n{} match(es) found (including {} hidden) in {} file(s).",
                match_count,
                match_count - printed_count,
                file_count
            )?;
        } else {
            writeln!(
                writer,
                "\n{} match(es) found in {} file(s).",
                match_count, file_count
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use self::cli::CommandLineArgs;
    use super::*;
    use std::io::stdout;
    use std::path::PathBuf;

    fn create_test_config(pattern: &str, case_insensitive: bool) -> CommandLineArgs {
        CommandLineArgs {
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
            files_with_matches: false,
            files_without_match: false,
        }
    }

    #[test]
    fn run_should_not_panic() -> Result<(), String> {
        let config = create_test_config("run", true);
        run(config, &mut stdout()).unwrap();
        Ok(())
    }
}
