use clap::Parser;
use becho::{
    Arguments,
    colors::Colors,
};

fn main() {
    let arguments: Arguments = Arguments::parse();
    let result: String = arguments.text
        .color_foreground(&arguments.foreground_color);
    eprintln!("{}", result);
}
