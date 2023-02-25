use clap::Parser;

pub mod styles;
pub mod io;
pub mod treatments;
pub mod error;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Arguments {
    /// Escapes new line and tab sequences.
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

    /// A color to be applied in the text.
    #[arg(
        short = 'f',
        long = "foreground-color",
        default_value_t = String::from("normal"),
    )]
    pub foreground_color: String,

    /// A color to be applied in the background.
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

    /// The separator used to concatenate text fragments.
    #[arg(
        short = 't',
        long = "separator",
        default_value_t = String::from(" "),
    )]
    pub separator: String,

    /// The case the text will be treated to.
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

    /// Prints useful information for debugging into the standart error.
    #[arg(
        short = 'v',
        long = "verbose",
        action,
    )]
    pub is_verbose: bool,

    /// The text fragments to be handled.
    pub text_fragments: Vec<String>,
}
