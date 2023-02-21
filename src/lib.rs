use clap::Parser;

pub mod colors;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// A color to be applied in the text.
    #[arg(short = 'f', long = "foreground-color", default_value_t = String::from("normal"))]
    pub foreground_color: String,
    /// The text to be handled.
    pub text: String,
}
