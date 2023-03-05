use clap::Parser;
use becho::{
    Arguments,
    styles::Styles,
    treatments::Treatments,
    io::{print_to_stdout, create_text},
};

fn main() {
    let arguments: Arguments = Arguments::parse();
    let text: String = create_text(
        &arguments.text_fragments,
        &arguments.separator,
        arguments.is_to_ignore_empty,
    );

    if arguments.is_verbose {
        eprintln!("{:#?}", arguments);
    }
    
    let has_style: bool =
        arguments.is_bold ||
        arguments.is_crossed_out ||
        arguments.is_italic ||
        arguments.is_underline ||
        arguments.foreground_color != "normal" ||
        arguments.background_color != "normal" ||
        arguments.is_dimmed;

    let output: String = text
        .treat_case(&arguments.case)
        .bold(arguments.is_bold)
        .cross_out(arguments.is_crossed_out)
        .italicize(arguments.is_italic)
        .underline(arguments.is_underline)
        .color_foreground(&arguments.foreground_color)
        .color_background(&arguments.background_color)
        .dim(arguments.is_dimmed)
        .add_end_sequence(has_style)
        .treat_width_and_sides(
            arguments.width,
            &arguments.left_indentation,
        )
        .escape_sequences(arguments.is_to_escape);
    print_to_stdout(
        output,
        arguments.number_of_repetitions,
        arguments.is_no_end_new_line,
    );
}
