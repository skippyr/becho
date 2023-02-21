use clap::Parser;

pub mod styles;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Uses bold text.
    #[arg(short = 'b', long = "bold", action)]
    pub is_bold: bool,
    /// A color to be applied in the text.
    #[arg(short = 'f', long = "foreground-color", default_value_t = String::from("normal"))]
    pub foreground_color: String,
    /// The text to be handled.
    pub text: String,
}
