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
