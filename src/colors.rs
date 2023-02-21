use crossterm::style::Stylize;

pub fn color_foreground(foreground_color: &str, text: &str) -> String {
  match foreground_color {
      "black" => {
          text.black().to_string()
      }
      "dark_red" => {
          text.dark_red().to_string()
      }
      "red" => {
          text.red().to_string()
      }
      "dark_green" => {
          text.dark_green().to_string()
      }
      "green" => {
          text.green().to_string()
      }
      "dark_yellow" => {
          text.dark_yellow().to_string()
      }
      "yellow" => {
          text.yellow().to_string()
      }
      "dark_blue" => {
          text.dark_blue().to_string()
      }
      "blue" => {
          text.blue().to_string()
      }
      "dark_magenta" => {
          text.dark_magenta().to_string()
      }
      "magenta" => {
          text.magenta().to_string()
      }
      "dark_cyan" => {
          text.dark_cyan().to_string()
      }
      "cyan" => {
          text.cyan().to_string()
      }
      "white" => {
          text.white().to_string()
      }
      _ => {
          text.to_string()
      }
  }
}
