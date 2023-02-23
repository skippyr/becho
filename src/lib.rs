use clap::Parser;

pub mod styles;
pub mod io;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Uses bold text.
    #[arg(short = 'b', long = "bold", action)]
    pub is_bold: bool,
    /// Uses crossed out text.
    #[arg(short = 'x', long = "crossed-out", action)]
    pub is_crossed_out: bool,
    /// A color to be applied in the text.
    #[arg(short = 'f', long = "foreground-color", default_value_t = String::from("normal"))]
    pub foreground_color: String,
    /// A color to be applied in the background.
    #[arg(short = 'g', long = "background-color", default_value_t = String::from("normal"))]
    pub background_color: String,
    /// The separator used to concatenate text fragments.
    #[arg(long = "separator", default_value_t = String::new())]
    pub separator: String,
    /// The text fragments to be handled.
    pub text_fragments: Vec<String>,
}
