use clap::Parser;
use becho::{
    Arguments,
    styles::Styles,
    treatments::Treatments,
    io::print_to_stdout,
};

fn main() {
    let arguments: Arguments = Arguments::parse();
    let text: String = arguments.text_fragments.join(&arguments.separator);

    if arguments.is_verbose {
        eprintln!("{:#?}", arguments);
    }

    let output: String = text
        .escape_sequences(arguments.is_to_escape)
        .treat_case(&arguments.case)
        .bold(arguments.is_bold)
        .cross_out(arguments.is_crossed_out)
        .italicize(arguments.is_italic)
        .underline(arguments.is_underline)
        .color_foreground(&arguments.foreground_color)
        .color_background(&arguments.background_color)
        .dim(arguments.is_dimmed)
        .treat_width_and_sides(
            arguments.width,
            &arguments.left_indentation,
        );
    print_to_stdout(
        output,
        arguments.number_of_repetitions,
        arguments.is_no_end_new_line,
    );
}
