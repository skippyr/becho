use crossterm::terminal::size;

pub fn get_terminal_width() -> usize {
    match size() {
        Ok(values) => { values.0 as usize }
        Err(_) => { 80 }
    }
}
