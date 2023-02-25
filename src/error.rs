use std::process::exit;

pub fn exit_process(message: String, exit_code: i32) {
  eprintln!("becho: {}", message);
  exit(exit_code);
}
