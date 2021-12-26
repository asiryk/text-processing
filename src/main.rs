#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::read_to_string;

lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"[ \n\r\t]+").unwrap(); // find all whitespaces
}

const FILE_PATH: &str = "assets/beyond-good-and-evil.txt";

fn main() {
    let mut text = read_to_string(FILE_PATH).unwrap();
    let text = normalize(&mut text);
    println!("{}", text.len());
}

fn normalize(text: &mut String) -> String {
    WHITESPACE.replace_all(text, " ").to_lowercase()
}

#[cfg(test)]
mod tests {
    use crate::normalize;

    #[test]
    fn replace_whitespaces_and_lowercase() {
        let mut text = String::from(
            "SOME
            tExt  \n\r\r\r\t\t  THat wiLL
    be normalized",
        );
        let result = String::from("some text that will be normalized");

        assert_eq!(result, normalize(&mut text));
    }
}
