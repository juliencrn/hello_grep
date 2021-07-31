use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "hello_grep",
    about = "A command line mini grep clone app written in Rust."
)]
pub struct CommandLineArgs {
    pub pattern: String,

    #[structopt(parse(from_os_str))]
    pub path: Vec<PathBuf>,

    #[structopt(
        short = "i",
        long = "ignore-case",
        help = "Make search case insensitive"
    )]
    pub case_insensitive: bool,

    #[structopt(short = "n", long = "line-number", help = "Show line number.")]
    pub show_line_number: bool,

    #[structopt(long = "color", help = "Activate color in output.")]
    pub display_color: bool,

    #[structopt(
        short = "v",
        long = "invert-match",
        help = "Invert the sense of matching."
    )]
    pub invert_match: bool,

    #[structopt(
        short = "c",
        long = "count",
        help = "Suppress normal output; instead print a count of matching lines for each input file."
    )]
    pub count: bool,

    #[structopt(
        short = "s",
        long = "stats",
        help = "Display match statistics at the end."
    )]
    pub stats: bool,

    #[structopt(
        short = "x",
        long = "line-regexp",
        help = "Select only those matches that exactly match the whole line."
    )]
    pub line_regexp: bool,

    #[structopt(
        short = "m",
        long = "max-count",
        help = "Stop reading a file after NUM matching lines.",
        default_value = "1000"
    )]
    pub max: usize,

    #[structopt(
        short = "h",
        long = "no-filename",
        help = "Suppress the prefixing of file names on output. This is the default when there is only one file to search."
    )]
    pub no_filename: bool,

    #[structopt(
        short = "L",
        long = "files-without-match",
        help = "Suppress normal output; instead print the name of each input file from which no output would normally have been printed. The scanning will stop on the first match."
    )]
    pub files_without_match: bool,

    #[structopt(
        short = "l",
        long = "files-with-matches",
        help = "Suppress normal output; instead print the name of each input file from which output would normally have been printed. The scanning will stop on the first match."
    )]
    pub files_with_matches: bool,
}
