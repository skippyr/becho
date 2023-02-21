use clap::Parser;
use becho::{
    Arguments,
    colors::color_foreground,
};

fn main() {
    let arguments: Arguments = Arguments::parse();
    println!(
        "{}",
        color_foreground(
            &arguments.foreground_color,
            &arguments.text
        ),
    );
}
