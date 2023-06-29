use regex::Regex;

use crate::utils::is_hexa;

use crate::elements::Category;
use crate::errors::ParsingError;
use crate::utils::is_digit;
use crate::{elements::Elements, utils::is_hexa};
pub fn is_crc32(sub_tolen: &str) -> bool {
    is_hexa(sub_tolen) && sub_tolen.len() == 8
}

pub fn is_resolution(sub_tolen: &str) -> bool {
    let resolution_regex = Regex::new(r"\d{3,4}([pP]|([xX\u00D7]\d{3,4}))$|^[248]K$").unwrap();
    resolution_regex.is_match(sub_tolen)
}
