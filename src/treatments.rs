use convert_case::{Case, Casing};

pub trait Treatments {
    fn escape_sequences(&self, is_to_escape: bool) -> String;
    fn treat_case(&self, case: &str) -> String;
}

impl Treatments for String {
    fn escape_sequences(&self, is_to_escape: bool) -> String {
        if is_to_escape {
            self
                .clone()
                .replace("\\n", "\n")
                .replace("\\t", "  ")
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
                self.clone().to_case(Case::Camel)
            }
            "upper_camel" => {
                self.clone().to_case(Case::UpperCamel)
            }
            "title" => {
                self.clone().to_case(Case::Title)
            }
            "snake" => {
                self.clone().to_case(Case::Snake)
            }
            "upper_snake" => {
                self.clone().to_case(Case::ScreamingSnake)
            }
            "kebab" => {
                self.clone().to_case(Case::Kebab)
            }
            "upper_kebab" => {
                self.clone().to_case(Case::UpperKebab)
            }
            "title_kebab" => {
                self.clone().to_case(Case::Train)
            }
            "alternate" => {
                self.clone().to_case(Case::Alternating)
            }
            "invert" => {
                self.clone().to_case(Case::Toggle)
            }
            _ => {
                self.clone()
            }
        }
    }
}