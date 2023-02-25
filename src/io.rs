use std::io::{stdout, Write};

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
