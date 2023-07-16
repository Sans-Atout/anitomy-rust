use unicode_normalization::UnicodeNormalization;

pub fn remove_ignored_string(working_string: &str, ignored_str: Vec<String>) -> String {
    let mut return_string = working_string.to_string();
    for i_s in ignored_str {
        return_string = return_string.replace(&i_s, "");
    }
    return_string
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
