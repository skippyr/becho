use crossterm::style::{Color, Stylize};
use crate::error::exit_process;
pub trait Styles {
    fn bold(&self, is_bold: bool) -> String;
    fn cross_out(&self, is_crossed: bool) -> String;
    fn italicize(&self, is_italic: bool) -> String;
    fn underline(&self, is_underline: bool) -> String;
    fn color_foreground(&self, foreground_color: &str) -> String;
    fn color_background(&self, background_color: &str) -> String;
    fn dim(&self, is_dimmed: bool) -> String;
    fn reverse_characters(&self) -> String;
    fn remove_last_substring(&self, substring: &str) -> String;
    fn remove_end_sequences(&self) -> String;
    fn add_end_sequence(&self, is_to_add_end_sequence: bool) -> String;
}

impl Styles for String {
    fn bold(&self, is_bold: bool) -> String {
        if is_bold {
            self
                .split(" ")
                .map(|word| word.bold().to_string().remove_end_sequences())
                .collect::<Vec<String>>()
                .join(" ")
        } else {
            self.clone()
        }
    }

    fn cross_out(&self, is_crossed_text: bool) -> String {
        if is_crossed_text {
            self
                .split(" ")
                .map(|word| {
                    word.crossed_out().to_string().remove_end_sequences()
                })
                .collect::<Vec<String>>()
                .join(" ")
        } else {
            self.clone()
        }
    }

    fn italicize(&self, is_italic: bool) -> String {
        if is_italic {
            self
                .split(" ")
                .map(|word| {
                    word.italic().to_string().remove_end_sequences()
                })
                .collect::<Vec<String>>()
                .join(" ")
        } else {
            self.clone()
        }
    }

    fn color_foreground(&self, foreground_color: &str) -> String {
        let words = self.split(" ");
        let words_treated: Vec<String> =  match foreground_color {
            "black" => {
                words
                    .map(|word| {
                        word.black().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_red" => {
                words
                    .map(|word| {
                        word.dark_red().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "red" => {
                words
                    .map(|word| {
                        word.red().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_green" => {
                words
                    .map(|word| {
                        word.dark_green().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "green" => {
                words
                    .map(|word| {
                        word.green().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_yellow" => {
                words
                    .map(|word| {
                        word.dark_yellow().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "yellow" => {
                words
                    .map(|word| {
                        word.yellow().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_blue" => {
                words
                    .map(|word| {
                        word.dark_blue().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "blue" => {
                words
                    .map(|word| {
                        word.blue().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_magenta" => {
                words
                    .map(|word| {
                        word.dark_magenta().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "magenta" => {
                words
                    .map(|word| {
                        word.magenta().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_cyan" => {
                words
                    .map(|word| {
                        word.dark_cyan().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "cyan" => {
                words
                    .map(|word| {
                        word.cyan().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "white" => {
                words
                    .map(|word| {
                        word.white().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "normal" => {
                words
                    .map(|word| word.to_string())
                    .collect::<Vec<String>>()
            }
            _ => {
                match foreground_color.parse::<u8>() {
                    Ok(value) => {
                        words
                            .map(|word| {
                                word
                                    .with(Color::AnsiValue(value))
                                    .to_string()
                                    .remove_end_sequences()
                            })
                            .collect::<Vec<String>>()
                    }
                    Err(_) => {
                        exit_process(
                            format!(
                                "\"{}\" is not a valid foreground color.",
                                foreground_color
                            ),
                            1,
                        );
                        words
                            .map(|word| word.to_string())
                            .collect::<Vec<String>>()
                    }
                }
            }
        };
        words_treated.join(" ")
    }

    fn color_background(&self, background_color: &str) -> String {
        let words = self.split(" ");
        let words_treated: Vec<String> =  match background_color {
            "black" => {
                words
                    .map(|word| {
                        word.on_black().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_red" => {
                words
                    .map(|word| {
                        word.on_dark_red().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "red" => {
                words
                    .map(|word| {
                        word.on_red().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_green" => {
                words
                    .map(|word| {
                        word.on_dark_green().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "green" => {
                words
                    .map(|word| {
                        word.on_green().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_yellow" => {
                words
                    .map(|word| {
                        word.on_dark_yellow().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "yellow" => {
                words
                    .map(|word| {
                        word.on_yellow().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_blue" => {
                words
                    .map(|word| {
                        word.on_dark_blue().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "blue" => {
                words
                    .map(|word| {
                        word.on_blue().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_magenta" => {
                words
                    .map(|word| {
                        word.on_dark_magenta().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "magenta" => {
                words
                    .map(|word| {
                        word.on_magenta().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "dark_cyan" => {
                words
                    .map(|word| {
                        word.on_dark_cyan().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "cyan" => {
                words
                    .map(|word| {
                        word.on_cyan().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "white" => {
                words
                    .map(|word| {
                        word.on_white().to_string().remove_end_sequences()
                    })
                    .collect::<Vec<String>>()
            }
            "normal" => {
                words
                    .map(|word| word.to_string())
                    .collect::<Vec<String>>()
            }
            _ => {
                match background_color.parse::<u8>() {
                    Ok(value) => {
                        words
                            .map(|word| {
                                word
                                    .on(Color::AnsiValue(value))
                                    .to_string()
                                    .remove_end_sequences()
                            })
                            .collect::<Vec<String>>()
                    }
                    Err(_) => {
                        exit_process(
                            format!(
                                "\"{}\" is not a valid background color.",
                                background_color
                            ),
                            1,
                        );
                        words
                            .map(|word| word.to_string())
                            .collect::<Vec<String>>()
                    }
                }
            }
        };
        words_treated.join(" ")
    }

    fn underline(&self, is_underline: bool) -> String {
        if is_underline {
            self
                .split(" ")
                .map(|word| {
                    word.underlined().to_string().remove_end_sequences()
                })
                .collect::<Vec<String>>()
                .join(" ")
        } else {
            self.clone()
        }
    }

    fn dim(&self, is_dimmed: bool) -> String {
        if is_dimmed {
            self
                .split(" ")
                .map(|word| {
                    word.dim().to_string().remove_end_sequences()
                })
                .collect::<Vec<String>>()
                .join(" ")
        } else {
            self.clone()
        }
    }

    fn reverse_characters(&self) -> String {
        self
            .chars()
            .rev()
            .map(|character| character.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn remove_last_substring(&self, substring: &str) -> String {
        self
            .reverse_characters()
            .replacen(
                substring
                    .to_string()
                    .reverse_characters()
                    .as_str(),
                    "",
                    1
                )
            .reverse_characters()
    }

    fn remove_end_sequences(&self) -> String {
        self
            .remove_last_substring("\u{1b}[0m")
            .remove_last_substring("\u{1b}[39m")
            .remove_last_substring("\u{1b}[49m")
    }

    fn add_end_sequence(&self, is_to_add_end_sequence: bool) -> String {
        if is_to_add_end_sequence {
            format!("{}\u{1b}[0m", self)
        } else {
            self.to_string()
        }
    }
}
