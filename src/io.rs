use std::io::{Stdin, stdin, Read};

pub fn get_text_from_stdin() -> String {
    let mut standart_input_handler: Stdin = stdin();
    let mut buffer: String = String::new();
    standart_input_handler
        .read_to_string(&mut buffer)
        .expect("An error has happened when getting the text from stdin.");
    buffer.pop();
    buffer
}
