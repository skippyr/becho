//! Contains functions related to terminal's properties.

use crossterm::terminal::size;

/// Returns the terminal's width in characters. If it can not measure it, it
/// will return `80`.
pub fn get_terminal_width() -> usize {
    match size() {
        Ok(values) => { values.0 as usize }
        Err(_) => { 80 }
    }
}
