use regex::Regex;

const ANIME_YEAR_MIN: i32 = 1917;
const ANIME_YEAR_MAX: i32 = 2050;

pub fn is_crc32(sub_tolen: &str) -> bool {
    is_hexa(sub_tolen) && sub_tolen.len() == 8
}

pub fn is_resolution(sub_tolen: &str) -> bool {
    let resolution_regex = Regex::new(r"\d{3,4}([pP]|([xX\u00D7]\d{3,4}))$|^[248]K$").unwrap();
    resolution_regex.is_match(sub_tolen)
}

pub fn is_anime_year(tested_string: &str) -> bool {
    if !is_digit(tested_string) {
        return false;
    }
    let parsed_string = tested_string.parse::<i32>();
    if parsed_string.is_err() {
        return false;
    }
    let possible_year = parsed_string.unwrap();
    (ANIME_YEAR_MIN..=ANIME_YEAR_MAX).contains(&possible_year)
}

pub fn is_hexa(p_hexa: &str) -> bool {
    let hexa_char = Regex::new(r"^[0-9A-F]+$").unwrap();
    hexa_char.is_match(&p_hexa.to_uppercase())
}

pub fn is_digit(w: &str) -> bool {
    let digit_regex = Regex::new(r"^[-+0-9]+$").unwrap();
    digit_regex.is_match(&w.to_uppercase())
}

pub fn ordinals_to_nb(ordinal: &str) -> &str {
    let ordinal_raw = ordinal.to_lowercase();
    match ordinal_raw.as_str() {
        "1st" | "first" => "1",
        "2nd" | "second" => "2",
        "3rd" | "third" => "3",
        "4th" | "fourth" => "4",
        "5th" | "fifth" => "5",
        "6th" | "sixth" => "6",
        "7th" | "seventh" => "7",
        "8th" | "eighth" => "8",
        "9th" | "Ninth" => "9",
        _ => "",
    }
}

pub fn contains_digit(word: &str) -> bool {
    for char in word.chars() {
        if char.is_numeric() {
            return true;
        }
    }
    false
}
