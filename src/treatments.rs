pub trait Treatments {
    fn escape_spacing_sequences(&self, is_to_escape: bool) -> String;
}

impl Treatments for String {
    fn escape_spacing_sequences(&self, is_to_escape: bool) -> String {
        if is_to_escape {
            self
                .clone()
                .replace("\\n", "\n")
                .replace("\\t", "  ")
        } else {
            self.clone()
        }
    }
}