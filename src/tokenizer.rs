use crate::{
    elements::{Category, Elements},
    parsing::is_crc32,
    utils::{is_digit, split_by_delimiter},
};

pub fn tokenize(string_to_tokenize: &str, delimiter: &Vec<char>) -> Vec<Token> {
    let mut all_token: Vec<Token> = vec![];
    let token_char_vec: Vec<char> = string_to_tokenize.chars().collect();
    let mut index = 0;
    let mut tmp_token: Vec<char> = vec![];
    while index < token_char_vec.len() {
        let char = token_char_vec.get(index).unwrap();
        match char {
            '[' | '(' | '{' | '\u{300C}' | '\u{300E}' | '\u{3011}' | '\u{FF08}' => {
                if !tmp_token.is_empty() {
                    let token_str = tmp_token.iter().cloned().collect::<String>();
                    all_token.push(Token::new(&token_str, delimiter, false));
                }
                tmp_token = vec![];
            }
            ']' | ')' | '}' | '\u{300D}' | '\u{300F}' | '\u{3010}' | '\u{FF09}' => {
                if !tmp_token.is_empty() {
                    let token_str = tmp_token.iter().cloned().collect::<String>();
                    all_token.push(Token::new(&token_str, delimiter, true));
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
        let token_str = tmp_token.iter().cloned().collect::<String>();
        all_token.push(Token::new(&token_str, delimiter, false));
    }
    all_token
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    tokens: Vec<SubToken>,
    raw_token: String,
    inside_delimiter: bool,
}

impl Token {
    pub fn new(raw: &str, delimiter: &Vec<char>, in_delimiter: bool) -> Token {
        let splited_token = split_by_delimiter(raw, delimiter.to_owned());
        let mut all_tokens: Vec<SubToken> = Vec::new();
        for token in splited_token {
            all_tokens.push(SubToken::new(token.trim_matches(delimiter.as_slice())));
        }
        Token {
            tokens: all_tokens,
            raw_token: raw.to_string(),
            inside_delimiter: in_delimiter,
        }
    }

    pub fn parse(&self, e: &mut Elements) {
        for sub_token in self.tokens.clone() {
            if sub_token.value.is_empty() {
                continue;
            }
            if is_digit(&sub_token.value) && sub_token.value.len() != 8 {
                continue;
            }
            if is_crc32(&sub_token.value) && e.is_category_empty(Category::FileExtension) {
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
