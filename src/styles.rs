use crossterm::style::Stylize;

pub trait Styles {
    fn bold(&self, is_bold_text: bool) -> String;
    fn cross_out(&self, is_crossed_text: bool) -> String;
    fn color_foreground(&self, foreground_color: &str) -> String;
    fn color_background(&self, background_color: &str) -> String;
}

impl Styles for String {
    fn bold(&self, is_bold: bool) -> String {
        if is_bold {
            self.clone().bold().to_string()
        } else {
            self.clone()
        }
    }

    fn cross_out(&self, is_crossed_text: bool) -> String {
        if is_crossed_text {
            self.clone().crossed_out().to_string()
        } else {
            self.clone()
        }
    }

    fn color_foreground(&self, foreground_color: &str) -> String {
        match foreground_color {
            "black" => {
                self.clone().black().to_string()
            }
            "dark_red" => {
                self.clone().dark_red().to_string()
            }
            "red" => {
                self.clone().clone().red().to_string()
            }
            "dark_green" => {
                self.clone().dark_green().to_string()
            }
            "green" => {
                self.clone().green().to_string()
            }
            "dark_yellow" => {
                self.clone().dark_yellow().to_string()
            }
            "yellow" => {
                self.clone().yellow().to_string()
            }
            "dark_blue" => {
                self.clone().dark_blue().to_string()
            }
            "blue" => {
                self.clone().blue().to_string()
            }
            "dark_magenta" => {
                self.clone().dark_magenta().to_string()
            }
            "magenta" => {
                self.clone().magenta().to_string()
            }
            "dark_cyan" => {
                self.clone().dark_cyan().to_string()
            }
            "cyan" => {
                self.clone().cyan().to_string()
            }
            "white" => {
                self.clone().white().to_string()
            }
            _ => {
                self.clone()
            }
        }
    }

    fn color_background(&self, background_color: &str) -> String {
        match background_color {
            "black" => {
                self.clone().on_black().to_string()
            }
            "dark_red" => {
                self.clone().on_dark_red().to_string()
            }
            "red" => {
                self.clone().clone().on_red().to_string()
            }
            "dark_green" => {
                self.clone().on_dark_green().to_string()
            }
            "green" => {
                self.clone().on_green().to_string()
            }
            "dark_yellow" => {
                self.clone().on_dark_yellow().to_string()
            }
            "yellow" => {
                self.clone().on_yellow().to_string()
            }
            "dark_blue" => {
                self.clone().on_dark_blue().to_string()
            }
            "blue" => {
                self.clone().on_blue().to_string()
            }
            "dark_magenta" => {
                self.clone().on_dark_magenta().to_string()
            }
            "magenta" => {
                self.clone().on_magenta().to_string()
            }
            "dark_cyan" => {
                self.clone().on_dark_cyan().to_string()
            }
            "cyan" => {
                self.clone().on_cyan().to_string()
            }
            "white" => {
                self.clone().on_white().to_string()
            }
            _ => {
                self.clone()
            }
        }
    }
}
