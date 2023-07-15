use regex::Regex;

use crate::{utils::{is_hexa, is_digit}, tokenizer::SubToken, elements::Elements};
use crate::elements::Category;

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

pub fn is_anime_year(tested_string : &str) -> bool{
    if !is_digit(tested_string){
        return false;
    }
    let parsed_string = tested_string.parse::<i32>();
    if parsed_string.is_err(){
        return false
    }
    let possible_year = parsed_string.unwrap();
    (ANIME_YEAR_MIN..=ANIME_YEAR_MAX).contains(&possible_year)
}

impl SubToken {

    pub fn match_episode_patern(&self, found_elements : &mut Elements) -> bool {
        let tested_value = self.value();
        println!("{}",tested_value);

        // Multi episode matching test
        let multiple_ep_regex = Regex::new(r"(?P<ep_1>\d{1,4})(?:[vV](?P<version_1>\d))?[-~&+](?P<ep_2>\d{1,4})(?:[vV](?P<version_2>\d))?$").unwrap();
        if multiple_ep_regex.is_match(&tested_value){
            let multiple_ep_captures = multiple_ep_regex.captures(&tested_value).unwrap();
            let ep_1 = multiple_ep_captures.name("ep_1").unwrap().as_str();
            let ep_2 = multiple_ep_captures.name("ep_2").unwrap().as_str();
            if ep_1.parse::<i32>().unwrap() > ep_2.parse::<i32>().unwrap(){
                return false;
            }
            found_elements.add(Category::EpisodeNumber,  ep_1);
            found_elements.add(Category::EpisodeNumberAlt, ep_2);

            if let Some(v1) = multiple_ep_captures.name("version_1") {
                found_elements.add(Category::ReleaseVersion, v1.as_str());
            }
            if let Some(v2) = multiple_ep_captures.name("version_2") {
                found_elements.add(Category::ReleaseVersion, v2.as_str());
            }
            return true;
        }

        // Single episode matching test
        let single_ep_regex = Regex::new(r"(?P<episode>\d{1,4})[vV](?P<version>\d)$").unwrap();
        if single_ep_regex.is_match(&tested_value){
            let single_ep_captures = single_ep_regex.captures(&tested_value).unwrap();
            found_elements.add(Category::EpisodeNumber,  &single_ep_captures["episode"]);
            found_elements.add(Category::ReleaseVersion,  &single_ep_captures["version"]);
            return true;
        }

        false
    }
}

pub fn contains_digit(word : &str) -> bool{
    for char in word.chars(){
        if char.is_numeric() {
            return true;
        }
    }
    false
}