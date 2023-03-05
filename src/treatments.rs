//! Contains functions related to string treatments and text wrapping.

use convert_case::{Case, Casing};
use textwrap::{
    wrap,
    core::display_width,
};
use crossterm::style::Stylize;
use crate::error::exit_process;

/// A trait that contains treatments to be implemented for the `String` type.
pub trait Treatments {
    /// Replaces double backslashes (`\\`) of new line (`\n`), tab (`\t`) and
    /// escape (`\e`, `\x1b` and `\003`) characters that is in a string to only
    /// one backslash `\`, making those be escaped when the string is printed.
    fn escape_sequences(&self, is_to_escape: bool) -> String;
    
    /// Treats the case of a string based on the value of the argument `case`.
    ///
    ///
    /// ### Panics
    /// 
    /// It panics and exits the main process if the `case` value is considered
    /// invalid.
    fn treat_case(&self, case: &str) -> String;

    /// Adds left indentation and wraps a String type based on the value of
    /// the `width` argument.
    /// 
    /// 
    /// ### Panics
    /// 
    /// It panics and exits the main process if it detects that all the text's
    /// elements can not fit in the `width` value provided.
    fn treat_width_and_sides(
        &self,
        width: usize,
        left_indentation: &str,
    ) -> String;
}

impl Treatments for String {
    fn escape_sequences(&self, is_to_escape: bool) -> String {
        if is_to_escape {
            self
                .clone()
                .replace("\\t", "  ")
                .replace("\\n", "\n")
                .replace("\\e", "\x1b")
                .replace("\\x1b", "\x1b")
                .replace("\\033", "\x1b")
        } else {
            self.clone()
        }
    }

    fn treat_case(&self, case: &str) -> String {
        match case {
            "upper" => {
                self.clone().to_uppercase()
            }
            "lower" => {
                self.clone().to_lowercase()
            }
            "camel" => {
                self.clone().to_lowercase().to_case(Case::Camel)
            }
            "upper_camel" => {
                self.clone().to_lowercase().to_case(Case::UpperCamel)
            }
            "title" => {
                self.clone().to_lowercase().to_case(Case::Title)
            }
            "snake" => {
                self.clone().to_lowercase().to_case(Case::Snake)
            }
            "upper_snake" => {
                self.clone().to_lowercase().to_case(Case::ScreamingSnake)
            }
            "kebab" => {
                self.clone().to_lowercase().to_case(Case::Kebab)
            }
            "upper_kebab" => {
                self.clone().to_lowercase().to_case(Case::UpperKebab)
            }
            "title_kebab" => {
                self.clone().to_lowercase().to_case(Case::Train)
            }
            "alternate" => {
                self.clone().to_lowercase().to_case(Case::Alternating)
            }
            "normal" => {
                self.clone()
            }
            _ => {
                exit_process(
                    format!("\"{}\" is not a valid case.", case),
                    1
                );
                self.clone()
            }
        }
    }

    fn treat_width_and_sides(
            &self,
            width: usize,
            left_indentation: &str,
        ) -> String {
        let text_width: usize =
            width
            - display_width(left_indentation);
        if text_width > width {
            exit_process(
                format!("can not fit text in width \"{}\".", width),
                1,
            )
        }
        let lines = wrap(&self, text_width);
        let mut lines_treated: Vec<String> = Vec::new();
        for line in lines {
            lines_treated.push(
                format!(
                    "{}{}",
                    left_indentation,
                    line,
                )
            )
        }
        lines_treated.join("\n".reset().to_string().as_str())
    }
}
