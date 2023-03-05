//! Contains functions related to input and output processes.

use std::io::{stdout, Write};
use textwrap::core::display_width;

/// Prints an output to the standart output a certain number of times. Its
/// behavior changes depending on the value of the boolean argument
/// `is_no_end_new_line`.
/// 
/// + If it is true, the output will be pushed to the standart output and then
///   the buffer is flushed, making it be outputted without a new line character
///   in the end.
/// + Else, it will be printed by using the macro `println!`.
/// 
/// 
/// ### Examples
///
/// ```
/// use becho::io::print_to_stdout;
/// 
/// print_to_stdout("Here Are Dragons!".to_string(), 2, true);
/// ```
pub fn print_to_stdout(
    output: String,
    number_of_repetitions: usize,
    is_no_end_new_line: bool,
) {
    for _ in 0..number_of_repetitions {
        if is_no_end_new_line {
            print!("{}", output);
            stdout()
                .flush()
                .expect("An error has happened when flushing the stdout.");
        } else {
            println!("{}", output);
        }
    }
}

/// Creates the text that will be handled by the main process by parsing
/// the text fragments received as command line arguments based on the values
/// of `separator` and `is_to_ignore_empty`.
pub fn create_text(
    text_fragments: &Vec<String>,
    separator: &str,
    is_to_ignore_empty: bool,
) -> String {
    if is_to_ignore_empty {
        let mut non_empty_text_fragments: Vec<String> = Vec::new();

        for text_fragment in text_fragments.clone() {
            if display_width(text_fragment.trim()) > 0 {
                non_empty_text_fragments.push(text_fragment)
            }
        }

        non_empty_text_fragments.join(separator)
    } else {
        text_fragments.join(separator)
    }
}
