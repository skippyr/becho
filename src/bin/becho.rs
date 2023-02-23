use clap::Parser;
use becho::{
    Arguments,
    styles::Styles,
    treatments::Treatments,
    io::get_text_from_stdin,
};

fn main() {
    let arguments: Arguments = Arguments::parse();
    let text: String = if arguments.text_fragments.len() > 0 {
        arguments.text_fragments.join(&arguments.separator)
    } else {
        get_text_from_stdin()
    };

    let output: String = text
        .escape_spacing_sequences(arguments.is_to_escape)
        .bold(arguments.is_bold)
        .cross_out(arguments.is_crossed_out)
        .italicize(arguments.is_italic)
        .color_foreground(&arguments.foreground_color)
        .color_background(&arguments.background_color)
        .dim(arguments.is_dimmed);
    println!("{}", output);
}
