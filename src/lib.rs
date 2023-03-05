use clap::Parser;
use terminal::get_terminal_width;

pub mod styles;
pub mod io;
pub mod treatments;
pub mod error;
pub mod terminal;
pub mod tests;

#[derive(Debug, Parser)]
#[clap(version)]
/// An struct that contains all the possible command line arguments and its
/// values.
pub struct Arguments {
    /// Escapes new line (`\n`), tab (`\t`) and escape (`\e`, `\x1b` and `\033`)
    /// characters.
    #[arg(short = 'e', long = "escape", action)]
    pub is_to_escape: bool,

    /// Uses bold text.
    #[arg(short = 'b', long = "bold", action)]
    pub is_bold: bool,

    /// Uses underlined text.
    #[arg(
        short = 'u',
        long = "underline",
        action,
    )]
    pub is_underline: bool,

    /// Uses crossed out text.
    #[arg(short = 'x', long = "crossed-out", action)]
    pub is_crossed_out: bool,

    /// Uses italic text.
    #[arg(short = 'i', long = "italic", action)]
    pub is_italic: bool,

    /// Defines the foreground color.
    #[arg(
        short = 'f',
        long = "foreground-color",
        default_value_t = String::from("normal"),
    )]
    pub foreground_color: String,

    /// Defines the background color.
    #[arg(
        short = 'g',
        long = "background-color",
        default_value_t = String::from("normal"),
    )]
    pub background_color: String,

    /// Uses dimmed color.
    #[arg(
        short = 'd',
        long = "dim",
        action,
    )]
    pub is_dimmed: bool,

    /// Defines the separator string to be used when concatenating the text
    /// fragments given as arguments.
    #[arg(
        short = 't',
        long = "separator",
        default_value_t = String::from(" "),
    )]
    pub separator: String,

    /// Defines the case the text will be treated to.
    #[arg(
        short = 'c',
        long = "case",
        default_value_t = String::from("normal"),
    )]
    pub case: String,

    /// Do not use a new line character in the end of output.
    #[arg(
        short = 'n',
        long = "no-end-new-line",
        action,
    )]
    pub is_no_end_new_line: bool,

    /// Defines the number of times the output will be repeated.
    #[arg(
        short = 'R',
        long = "repeat",
        default_value_t = 1,
    )]
    pub number_of_repetitions: usize,

    /// Ignores text fragments that, if trimmed, are an empty string.
    #[arg(
        short = 'I',
        long = "ignore-empty",
        action,
    )]
    pub is_to_ignore_empty: bool,

    /// Defines a string to be used as left indentation.
    #[arg(
        short = 'l',
        long = "left-indentation",
        default_value_t = String::new(),
    )]
    pub left_indentation: String,

    /// Defines the width of the output.
    #[arg(
        short = 'w',
        long = "width",
        default_value_t = get_terminal_width(),
    )]
    pub width: usize,

    /// Prints useful information for debugging to the standard error.
    #[arg(
        short = 'v',
        long = "verbose",
        action,
    )]
    pub is_verbose: bool,

    /// The text fragments to be handled.
    pub text_fragments: Vec<String>,
}
