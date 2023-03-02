use clap::Parser;
use becho::{
    Arguments,
    styles::Styles,
    treatments::Treatments,
    io::print_to_stdout,
};
use textwrap::core::display_width;

fn main() {
    let arguments: Arguments = Arguments::parse();
    let text: String = if arguments.is_to_ignore_empty {
        let mut non_empty_text_fragments = Vec::new();

        for text_fragment in arguments.text_fragments.clone() {
            if display_width(text_fragment.trim()) > 0 {
                non_empty_text_fragments.push(text_fragment)
            }
        }

        non_empty_text_fragments.join(&arguments.separator)
    } else {
        arguments.text_fragments.join(&arguments.separator)
    };

    if arguments.is_verbose {
        eprintln!("{:#?}", arguments);
    }

    let output: String = text
        .treat_case(&arguments.case)
        .bold(arguments.is_bold)
        .cross_out(arguments.is_crossed_out)
        .italicize(arguments.is_italic)
        .underline(arguments.is_underline)
        .color_foreground(&arguments.foreground_color)
        .color_background(&arguments.background_color)
        .dim(arguments.is_dimmed)
        .add_end_sequence(
            arguments.is_bold ||
            arguments.is_crossed_out ||
            arguments.is_italic ||
            arguments.is_underline ||
            arguments.foreground_color != "normal" ||
            arguments.background_color != "normal" ||
            arguments.is_dimmed
        )
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
