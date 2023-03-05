//! Contains functions related to input and output processes.

use std::io::{stdout, Write};

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
    is_no_end_new_line: bool
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
