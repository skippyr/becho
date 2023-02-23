use clap::Parser;
use becho::{
    Arguments,
    styles::Styles,
    io::get_text_from_stdin
};

fn main() {
    let arguments: Arguments = Arguments::parse();
    let text: String = if arguments.text_fragments.len() > 0 {
        arguments.text_fragments.concat()
    } else {
        get_text_from_stdin()
    };

    let output: String = text
        .bold(arguments.is_bold)
        .color_foreground(&arguments.foreground_color)
        .color_background(&arguments.background_color);
    println!("{}", output);
}
