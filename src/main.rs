#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::read_to_string;

lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();

    // Match all sentences containing the word with "philos"
    static ref SENTENCE_WITH_WORD: Regex = Regex::new(r"[^\.?!]*\bphilos[^\.?!]*[\.?!]").unwrap();
}

const FILE_PATH: &str = "assets/beyond-good-and-evil.txt";

fn main() {
    let text = read_to_string(FILE_PATH).unwrap();
    let text = normalize(&text);
    println!("{}", text.len());
    let matches = fd_sentences_with_word(&text);
    println!("{:#?}", matches.len());
}

fn normalize(text: &String) -> String {
    WHITESPACE.replace_all(text, " ").to_lowercase()
}

fn fd_sentences_with_word(text: &String) -> Vec<String> {
    SENTENCE_WITH_WORD
        .find_iter(text)
        .map(|mat| String::from(mat.as_str().trim()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{fd_sentences_with_word, normalize};

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

    #[test]
    fn match_all_containing_philos() {
        let text = String::from("philosophy is the. philosophers. are the philosopical reasons. sophilos abc. philosophy not");
        let actual = fd_sentences_with_word(&text);
        let expected = vec![
            "philosophy is the.",
            "philosophers.",
            "are the philosopical reasons.",
        ];

        assert_eq!(expected, actual);
    }
}
