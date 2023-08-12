use crate::{parsing::number::is_digit, split::split_token};

use super::subtoken::{SubToken, SubTokenCategory};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    tokens: Vec<SubToken>,
    raw_token: String,
    inside_delimiter: bool,
    weak_delimiter: bool,
}

impl Token {
    pub fn new(raw: &str, delimiter: &[char], in_delimiter: bool, is_weak: bool) -> Token {
        let splited_token = split_token(raw, delimiter);
        let mut all_tokens: Vec<SubToken> = Vec::new();
        for token in splited_token {
            all_tokens.push(SubToken::new(&token));
        }
        Token {
            weak_delimiter: is_weak && all_tokens.len() == 1,
            tokens: all_tokens,
            raw_token: raw.to_string(),
            inside_delimiter: in_delimiter,
        }
    }

    pub fn contains_unknow(&self) -> bool {
        for t in &self.tokens {
            if t.is_category(SubTokenCategory::Unknow) {
                return true;
            }
        }
        false
    }

    pub fn is_full_unknow(&self) -> bool {
        for t in &self.tokens {
            if !t.is_category(SubTokenCategory::Unknow) {
                return false;
            }
        }
        true
    }

    pub fn is_isolated_number(&self) -> bool {
        let first_token = self.tokens.get(0).unwrap();
        self.tokens.len() == 1
            && is_digit(&first_token.value())
            && !&first_token.is_category(SubTokenCategory::Found)
    }

    pub fn sub_tokens(&mut self) -> &mut Vec<SubToken> {
        &mut self.tokens
    }

    pub fn raw_token(&self) -> String {
        self.raw_token.to_owned()
    }

    pub fn is_inside_delimiter(&self) -> bool {
        self.inside_delimiter
    }

    pub fn is_weak(&self) -> bool {
        self.weak_delimiter
    }
}
