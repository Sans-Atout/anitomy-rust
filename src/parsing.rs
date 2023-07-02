use regex::Regex;

use crate::utils::is_hexa;

const ANIME_YEAR_MIN: i32 = 1917;
const ANIME_YEAR_MAX: i32 = 2050;
const EPISODE_NUMBER_MAX: i32 = ANIME_YEAR_MIN - 1;
const VOLUME_NUMBER_MAX: i32 = 20;

pub fn is_crc32(sub_tolen: &str) -> bool {
    is_hexa(sub_tolen) && sub_tolen.len() == 8
}

pub fn is_resolution(sub_tolen: &str) -> bool {
    let resolution_regex = Regex::new(r"\d{3,4}([pP]|([xX\u00D7]\d{3,4}))$|^[248]K$").unwrap();
    resolution_regex.is_match(sub_tolen)
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