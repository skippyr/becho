#[cfg(test)]
use crate::styles::Styles;

#[cfg(test)]
struct Color {
    name: String,
    value: i32,
}

#[cfg(test)]
impl Color {
    fn new(name: &str, value: i32) -> Self {
        let name: String = String::from(name);
        Color { name, value }
    }
}

#[cfg(test)]
fn get_text() -> String {
    String::from("Here Are Dragons!")
}

#[cfg(test)]
fn get_colors() -> Vec<Color> {
    vec![
        Color::new("black", 0),
        Color::new("light_black", 8),
        Color::new("red", 1),
        Color::new("light_red", 9),
        Color::new("green", 2),
        Color::new("light_green", 10),
        Color::new("yellow", 3),
        Color::new("light_yellow", 11),
        Color::new("blue", 4),
        Color::new("light_blue", 12),
        Color::new("magenta", 5),
        Color::new("cyan", 6),
        Color::new("white", 7),
        Color::new("light_white", 15),
    ]
}

#[test]
fn test_bold() {
    let text: String = get_text();
    assert_eq!(
        text.bold(true),
        text
        .split(" ")
        .map(|word| format!("\u{1b}[1m{}", word))
        .collect::<Vec<String>>()
        .join(" "),
        "testing if bold sequence is applied to each word of a string.",
    );
}

#[test]
fn test_cross_out() {
    let text: String = get_text();
    assert_eq!(
        text.cross_out(true),
        text
        .split(" ")
        .map(|word| format!("\u{1b}[9m{}", word))
        .collect::<Vec<String>>()
        .join(" "),
        "testing if crossed out sequence is applied to each word of a string.",
    );
}

#[test]
fn test_italicize() {
    let text: String = get_text();
    assert_eq!(
        text.italicize(true),
        text
        .split(" ")
        .map(|word| format!("\u{1b}[3m{}", word))
        .collect::<Vec<String>>()
        .join(" "),
        "testing if italic sequence is applied to each word of a string.",
    );
}

#[test]
fn test_underline() {
    let text: String = get_text();
    assert_eq!(
        text.underline(true),
        text
        .split(" ")
        .map(|word| format!("\u{1b}[4m{}", word))
        .collect::<Vec<String>>()
        .join(" "),
        "testing if underline sequence is applied to each word of a string.",
    );
}

#[test]
fn test_dim() {
    let text: String = get_text();
    assert_eq!(
        text.dim(true),
        text
        .split(" ")
        .map(|word| format!("\u{1b}[2m{}", word))
        .collect::<Vec<String>>()
        .join(" "),
        "testing if dim sequence is applied to each word of a string.",
    );
}

#[test]
fn test_foreground_colors() {
    let text: String = get_text();
    let colors: Vec<Color> = get_colors();
    for color_value in 0..255 {
        assert_eq!(
            text.color_foreground(format!("{}", color_value).as_str()),
            text
            .split(" ")
            .map(|word| format!("\u{1b}[38;5;{}m{}", color_value, word))
            .collect::<Vec<String>>()
            .join(" "),
            "testing if foreground color using value is applied to each word \
            of a string.",
        );
    }
    for color in colors {
        assert_eq!(
            text.color_foreground(&color.name),
            text
            .split(" ")
            .map(|word| format!("\u{1b}[38;5;{}m{}", color.value, word))
            .collect::<Vec<String>>()
            .join(" "),
            "testing if foreground color using name is applied to each word of \
            a string.",
        );
    }
}

#[test]
fn test_background_colors() {
    let text: String = get_text();
    let colors: Vec<Color> = get_colors();
    for color_value in 0..255 {
        assert_eq!(
            text.color_background(format!("{}", color_value).as_str()),
            text
            .split(" ")
            .map(|word| format!("\u{1b}[48;5;{}m{}", color_value, word))
            .collect::<Vec<String>>()
            .join(" "),
            "testing if background color using value is applied to each word \
            of a string.",
        );
    }
    for color in colors {
        assert_eq!(
            text.color_background(&color.name),
            text
            .split(" ")
            .map(|word| format!("\u{1b}[48;5;{}m{}", color.value, word))
            .collect::<Vec<String>>()
            .join(" "),
            "testing if background color using name is applied to each word \
            of a string.",
        );
    }
}

#[test]
fn test_reverse_characters() {
    let text: String = String::from("test");
    assert_eq!(
        text.reverse_characters(),
        String::from("tset"),
        "testing if string has characters reversed.",
    );
}

#[test]
fn test_remove_last_substring() {
    let text: String = String::from("test\t\n\n");
    assert_eq!(
        text.remove_last_substring("\n"),
        String::from("test\t\n"),
        "testing if last substring in a string is removed.",
    );
}

#[test]
fn test_remove_end_sequences() {
    let text: String = String::from("\u{1b}[38;5;1m\u{1b}[4m\u{1b}[3m\u{1b}[9m\
    \u{1b}[1mtest\u{1b}[0m\u{1b}[49m\u{1b}[39m\u{1b}[0m");
    assert_eq!(
        text.remove_end_sequences(),
        String::from("\u{1b}[38;5;1m\u{1b}[4m\u{1b}[3m\u{1b}[9m\u{1b}[1mtest\
        \u{1b}[0m"),
        "testing if last style end sequences are removed.",
    );
}

#[test]
fn test_add_end_sequence() {
    let text: String = String::from("\u{1b}[1mtest");
    assert_eq!(
        text.add_end_sequence(true),
        String::from("\u{1b}[1mtest\u{1b}[0m"),
        "testing if style end sequences is added.",
    );
}
