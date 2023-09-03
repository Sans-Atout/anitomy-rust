use unicode_normalization::UnicodeNormalization;

use crate::{chunk::Chunk, traits::ExtendedString};

impl ExtendedString for str {
    fn remove_ignored(&self, ignored: &[String]) -> String {
        let mut return_value = self.to_string();
        for string in ignored {
            return_value = return_value.replace(string, "");
        }
        return_value
    }

    fn normalize(&self) -> String {
        self.nfkd()
            .filter(|c| c.is_ascii())
            .collect::<String>()
            .to_uppercase()
    }
}

pub fn normalize(to_normalize: &str) -> String {
    let all_char = to_normalize.nfkd();
    let mut normalized_char: Vec<char> = vec![];
    for c in all_char {
        if c.is_ascii() {
            normalized_char.push(c);
        }
    }
    normalized_char
        .iter()
        .cloned()
        .collect::<String>()
        .to_uppercase()
}
