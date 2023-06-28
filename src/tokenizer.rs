use crate::{utils::{split_by_delimiter, is_digit}, elements::{Elements, Category}, parsing::is_crc32};

pub fn tokenize(string_to_tokenize: &str) -> Vec<String> {
    let mut all_token: Vec<String> = vec![];
    let token_char_vec: Vec<char> = string_to_tokenize.chars().collect();
    let mut index = 0;
    let mut tmp_token: Vec<char> = vec![];
    while index < token_char_vec.len() {
        let char = token_char_vec.get(index).unwrap();
        match char {
            '[' | ']' | '(' | ')' | '{' | '}' | '\u{300C}' | '\u{300D}' | '\u{300E}'
            | '\u{300F}' | '\u{3011}' | '\u{3010}' | '\u{FF08}' | '\u{FF09}' => {
                if !tmp_token.is_empty() {
                    let token = tmp_token.iter().cloned().collect::<String>();
                    all_token.push(token);
                }
                tmp_token = vec![];
            }
            _ => {
                tmp_token.push(char.to_owned());
            }
        }
        index += 1;
    }
    if !tmp_token.is_empty() {
        let token = tmp_token.iter().cloned().collect::<String>();
        all_token.push(token);
    }
    all_token
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    tokens: Vec<SubToken>,
    raw_token: String,
}

impl Token {
    pub fn new(raw: &str, delimiter: &Vec<char>) -> Token {
        let splited_token = split_by_delimiter(raw, delimiter.to_owned());
        let mut all_tokens: Vec<SubToken> = Vec::new();
        for token in splited_token {
            all_tokens.push(SubToken::new(token.trim_matches(delimiter.as_slice())));
        }
        Token {
            tokens: all_tokens,
            raw_token: raw.to_string(),
        }
    }

    pub fn parse(&self, e : &mut Elements){
        for sub_token in self.tokens.clone() {
            if sub_token.value.is_empty() {
                continue;
            }
            if is_digit(&sub_token.value) && sub_token.value.len() != 8 {
                continue;
            }
            if is_crc32(&sub_token.value) && e.is_category_empty(Category::FileExtension){
                e.add(Category::FileExtension, &sub_token.value);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubToken {
    value: String,
    category: SubTokenCategory,
}

impl SubToken {
    pub fn new(v: &str) -> SubToken {
        SubToken {
            value: v.to_string(),
            category: SubTokenCategory::default(),
        }
    }

    pub fn category(&mut self, c: SubTokenCategory) {
        self.category = c;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubTokenCategory {
    #[default]
    Unknow,
    Delimiter,
    Invalid,
}
