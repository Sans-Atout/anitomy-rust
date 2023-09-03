use itertools::Itertools;
use regex::Regex;

use crate::elements::Category;
use crate::{
    chunk::{Chunk, Status},
    elements::Elements,
    traits::{ChunksManipulation, ParsingNumber},
};
const ANIME_YEAR_MIN: i32 = 1917;
const ANIME_YEAR_MAX: i32 = 2050;

impl ParsingNumber for str {
    fn is_crc32(&self) -> bool {
        self.len() == 8 && self.is_hexa()
    }

    fn is_resolution(&self) -> bool {
        let resolution_regex = Regex::new(r"\d{3,4}([pP]|([xX\u00D7]\d{3,4}))$|^[248]K$").unwrap();
        resolution_regex.is_match(self)
    }

    fn is_hexa(&self) -> bool {
        let hexa_regex = Regex::new(r"^[0-9A-F]+$").unwrap();
        hexa_regex.is_match(&self.to_uppercase())
    }

    fn is_digit(&self) -> bool {
        self.parse::<i32>().is_ok()
    }

    fn is_anime_year(&self) -> bool {
        if !self.is_digit() {
            return false;
        }
        if let Ok(p_year) = self.parse::<i32>() {
            return (ANIME_YEAR_MIN..=ANIME_YEAR_MAX).contains(&p_year);
        }
        false
    }

    fn ordinals_to_nb(&self) -> &str {
        match self.to_lowercase().as_str() {
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

    fn contains_digit(&self) -> bool {
        self.chars().any(|char| char.is_numeric())
    }
}

pub fn parsing_isolated_number(e: &mut Elements, isolated: &[usize], chunks: &mut [Chunk]) {
    println!("||||||||||||||||||||||||||||||||||||||||||||||||||||||||");
    println!("{:?}",isolated);
    for id in isolated {
        let tested = chunks[*id].value();
        println!("tested : {}", tested);
        if chunks[*id].depth() > 0 {
            if tested.is_anime_year() && e.is_category_empty(Category::AnimeYear) {
                e.add(Category::AnimeYear, &tested);
                chunks[*id].found();
                continue;
            }
            if (&tested == "480" || &tested == "720" || &tested == "1080") && e.is_category_empty(Category::VideoResolution){
                e.add(Category::VideoResolution, &tested);
                chunks[*id].found();
                continue;
            }
        }
        if e.is_category_empty(Category::EpisodeNumber){
            e.add(Category::EpisodeNumber, &tested);
            chunks[*id].found();
        }
    }
    println!("||||||||||||||||||||||||||||||||||||||||||||||||||||||||");
}

#[cfg(test)]
pub mod test {
    use crate::traits::ParsingNumber;

    #[test]
    fn crc32() {
        assert!("C09462E2".is_crc32());
        assert!("8F59F2BA".is_crc32());
        assert!(!"8F5GF2BA".is_crc32());
        assert!(!"8F50F2BAA".is_crc32());
        assert!(!"8F50F2B".is_crc32());
    }

    #[test]
    fn resolution() {
        assert!("1080p".is_resolution());
        assert!("2K".is_resolution());
        assert!("1920x1080".is_resolution());
        assert!("720p".is_resolution());
        assert!(!"Hello world".is_resolution());
        assert!("1080×720".is_resolution());
    }

    #[test]
    fn ordinal() {
        assert_eq!("First".ordinals_to_nb(), "1");
        assert_eq!("second".ordinals_to_nb(), "2");
        assert_eq!("5th".ordinals_to_nb(), "5");
        assert_eq!("5the".ordinals_to_nb(), "");
    }

    #[test]
    fn anime_year() {
        assert!("2009".is_anime_year());
        assert!("1920".is_anime_year());
        assert!(!"1400".is_anime_year());
        assert!("2050".is_anime_year());
        assert!(!"2500".is_anime_year());
        assert!(!"gdyuighjdhgj".is_anime_year());
    }

    #[test]
    fn test_contains_number() {
        assert!("E01S02".contains_digit());
        assert!("032".contains_digit());
        assert!(!"A".contains_digit());
        assert!(!"Hello World".contains_digit());
    }

    #[test]
    fn test_hexa() {
        assert!("028934".is_hexa());
        assert!("FFF".is_hexa());
        assert!("0123456789ABCDEF".is_hexa());
        assert!(!"G015021".is_hexa());
        assert!("acbdef".is_hexa());
        assert!(!"00000fg".is_hexa())
    }

    #[test]
    fn test_isdigit() {
        assert!(!"FFF".is_digit());
        assert!("256".is_digit());
        assert!("-120".is_digit());
        assert!(!"Hello World".is_digit());
    }
}
