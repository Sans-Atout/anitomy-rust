use crate:: utils::is_hexa;

use crate::elements::Category;
use crate::errors::ParsingError;
use crate::utils::is_digit;
use crate::{elements::Elements, utils::is_hexa};

pub fn is_crc32(crc32: &str) -> bool {
    is_hexa(crc32) && crc32.len() == 8
}