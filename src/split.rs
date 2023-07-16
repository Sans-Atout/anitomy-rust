use regex::Regex;

pub fn split_by_delimiter(raw: &str, delimiter: Vec<char>) -> Vec<&str> {
    let mut raw_string = String::default();
    for s in delimiter {
        raw_string = format!(r"{}\{}", raw_string, s);
    }
    let regex_split_str = format!(r"([{}])", raw_string);
    let split_regex = Regex::new(&regex_split_str).unwrap();
    split_regex.split(raw).collect::<Vec<&str>>()
}

pub fn split_type_and_ep(to_parse: &str) -> (String, String) {
    let mut keyword_char: Vec<char> = Vec::default();
    let mut to_parse_char: Vec<char> = Vec::default();
    let mut has_pass_digit = false;

    for s_char in to_parse.chars() {
        if s_char.is_ascii_digit() {
            has_pass_digit = true;
        }

        if has_pass_digit {
            to_parse_char.push(s_char);
        } else {
            keyword_char.push(s_char);
        }
    }

    let keyword = keyword_char.iter().cloned().collect::<String>();
    let to_check = to_parse_char.iter().cloned().collect::<String>();
    (keyword, to_check)
}
