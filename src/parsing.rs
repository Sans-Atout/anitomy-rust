use error_stack::{Report, Result};

use crate::elements::Category;
use crate::errors::ParsingError;
use crate::utils::is_digit;
use crate::{elements::Elements, utils::is_hexa};

pub fn is_crc32(crc32: &str) -> bool {
    is_hexa(crc32) && crc32.len() == 8
}

pub fn parsing(
    tokens: Vec<String>,
    mut element: Elements,
    allowed_d: Vec<char>,
) -> Result<Elements, ParsingError> {
    for t in tokens {
        let token_to_test = t.trim_matches(allowed_d.as_slice());
        if token_to_test.is_empty() {
            continue;
        }
        if is_digit(token_to_test) && token_to_test.len() != 8 {
            continue;
        }

        if is_crc32(token_to_test) && element.is_category_empty(Category::FileChecksum) {
            element.add(Category::FileChecksum, &t);
        }
        // TODO implement other token verification
    }
    if element.is_empty() {
        return Err(Report::new(ParsingError::StringIsEmpty));
    }
    Ok(element)
}
