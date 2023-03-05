//! Contains functions related to error treatments and processes abortion.

use std::process::exit;

/// Exits the main process with an exit code and prints a message to the
/// standard error.
/// 
/// 
/// ### Panics
/// 
/// This function will make the main process panic to then exit it.
/// 
/// 
/// ### Examples
/// 
/// ```should_panic
/// use becho::error::exit_process;
/// 
/// exit_process("failed to get command line arguments.".to_string(), 1);
/// ```
pub fn exit_process(message: String, exit_code: i32) {
  eprintln!("becho: {}", message);
  exit(exit_code);
}
