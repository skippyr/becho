use std::io::{Stdin, stdin, Read, stdout, Write};

pub fn get_text_from_stdin() -> String {
    let mut standart_input_handler: Stdin = stdin();
    let mut buffer: String = String::new();
    standart_input_handler
        .read_to_string(&mut buffer)
        .expect("An error has happened when getting the text from stdin.");
    buffer.pop();
    buffer
}

pub fn print_to_stdout(output: String, is_no_end_new_line: bool) {
    if is_no_end_new_line {
        print!("{}", output);
        stdout()
            .flush()
            .expect("An error has happened when flushing the stdout.");
    } else {
        println!("{}", output);
    }
}
