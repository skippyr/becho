#[cfg(test)]
use crate::styles::Styles;

#[test]
fn test_bold() {
  let text: String = String::from("Here Are Dragons!");
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
  let text: String = String::from("Here Are Dragons!");
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
  let text: String = String::from("Here Are Dragons!");
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
  let text: String = String::from("Here Are Dragons!");
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
  let text: String = String::from("Here Are Dragons!");
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
