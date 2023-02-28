use convert_case::{Case, Casing};
use crate::error::exit_process;

pub trait Treatments {
    fn escape_sequences(&self, is_to_escape: bool) -> String;
    fn treat_case(&self, case: &str) -> String;
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
}
