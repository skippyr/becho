use clap::Parser;
use becho::{
    Arguments,
    styles::Styles,
};

fn main() {
    let arguments: Arguments = Arguments::parse();
    let result: String = arguments.text
        .bold(arguments.is_bold)
        .color_foreground(&arguments.foreground_color);
    println!("{}", result);
}
