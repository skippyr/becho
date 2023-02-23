use clap::Parser;
use becho::{
    Arguments,
    styles::Styles,
    io::get_text_from_stdin
};

fn main() {
    let mut arguments: Arguments = Arguments::parse();
    
    if arguments.text == "" {
        arguments.text = get_text_from_stdin();
    }
    
    let output: String = arguments.text
        .bold(arguments.is_bold)
        .color_foreground(&arguments.foreground_color)
        .color_background(&arguments.background_color);
    println!("{}", output);
}
