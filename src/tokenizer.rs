use crate::{
    elements::{Category, Elements},
    keyword::{Keyword, Manager},
    parsing::number::{is_crc32, is_digit, is_resolution, ordinals_to_nb},
    split::split_by_delimiter,
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

    pub fn parse(&mut self, e: &mut Elements) -> Elements {
        let keyword_manager = Manager::new();
        let mut tmp_elements = e.to_owned();
        for st_index in 0..self.tokens.len() {
            if self.tokens[st_index].value.is_empty() || self.tokens[st_index].is_found() {
                continue;
            }
            if is_digit(&self.tokens[st_index].value) && self.tokens[st_index].value.len() != 8 {
                continue;
            }
            if is_crc32(&self.tokens[st_index].value) && e.is_category_empty(Category::FileChecksum)
            {
                tmp_elements =
                    self.keyword_found(Category::FileChecksum, st_index, &mut tmp_elements);
                continue;
            }
            if is_resolution(&self.tokens[st_index].value) {
                tmp_elements =
                    self.keyword_found(Category::VideoResolution, st_index, &mut tmp_elements);
                continue;
            }

            if let Some(key_match) =
                keyword_manager.find(&self.tokens[st_index].value.to_uppercase())
            {
                tmp_elements = self.manage_found_keyword(key_match, st_index, &tmp_elements);
            }
        }
        tmp_elements.to_owned()
    }

    pub fn keyword_found(&mut self, c: Category, id: usize, e: &mut Elements) -> Elements {
        let mut tmp_elements = e.to_owned();
        tmp_elements = tmp_elements.add(c, &self.tokens[id].value);
        self.tokens[id].category = SubTokenCategory::Found;
        tmp_elements
    }

    pub fn manage_found_keyword(&mut self, keyword: &Keyword, id: usize, e: &Elements) -> Elements {
        let mut tmp_elements = e.to_owned();
        let tmp_category = keyword.get_category();
        if (!tmp_category.is_searchable())
            || (tmp_category.is_singular() && !e.is_category_empty(tmp_category))
        {
            return tmp_elements;
        }
        if tmp_category == Category::AnimeSeasonPrefix {
            if id as i32 - 1 > 0 {
                let previous = self.tokens[id - 1].clone();
                let ordinal_saeson = ordinals_to_nb(&previous.value);
                if !ordinal_saeson.is_empty() {
                    tmp_elements = tmp_elements.add(Category::AnimeSeason, ordinal_saeson);
                    self.tokens[id - 1].category = SubTokenCategory::Found;
                }
            }
            if id + 1 < self.tokens.len() {
                let next = self.tokens[id + 1].clone();
                if is_digit(&next.value) {
                    tmp_elements =
                        self.keyword_found(Category::AnimeSeason, id + 1, &mut tmp_elements);
                }
            }
            tmp_elements = self.keyword_found(keyword.get_category(), id, &mut tmp_elements);
            return tmp_elements;
        }
        if tmp_category == Category::EpisodePrefix {
            if id + 1 < self.tokens.len() {
                tmp_elements =
                    self.keyword_found(Category::EpisodeNumber, id + 1, &mut tmp_elements);
            }
            tmp_elements = self.keyword_found(Category::EpisodePrefix, id, &mut tmp_elements);
        }
        if tmp_category == Category::ReleaseVersion {
            let release_id = self.tokens[id]
                .value
                .clone()
                .to_lowercase()
                .replace('v', "");
            tmp_elements = tmp_elements.add(Category::ReleaseVersion, &release_id);
            self.tokens[id].category = SubTokenCategory::Found;
            return tmp_elements;
        }

        if tmp_category == Category::VolumePrefix {
            if id + 1 < self.tokens.len() {
                tmp_elements =
                    self.keyword_found(Category::VolumeNumber, id + 1, &mut tmp_elements);
            }
            tmp_elements = self.keyword_found(Category::VolumePrefix, id, &mut tmp_elements);
            return tmp_elements;
        }
        if tmp_category != Category::Unknown {
            tmp_elements = self.keyword_found(keyword.get_category(), id, &mut tmp_elements);
        }
        tmp_elements
    }

    pub fn contains_unknow(&self) -> bool {
        for t in &self.tokens {
            if t.category == SubTokenCategory::Unknow {
                return true;
            }
        }
        false
    }

    pub fn is_isolated_number(&self) -> bool {
        let first_token = self.tokens.get(0).unwrap();
        self.tokens.len() == 1 && is_digit(&first_token.value) && !&first_token.is_found()
    }

    pub fn sub_tokens(&mut self) -> &mut Vec<SubToken> {
        &mut self.tokens
    }

    pub fn is_inside_delimiter(&self) -> bool {
        self.inside_delimiter
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

    pub fn category(&mut self, c: SubTokenCategory) -> SubToken {
        self.category = c;
        self.to_owned()
    }

    pub fn is_found(&self) -> bool {
        self.category == SubTokenCategory::Found
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubTokenCategory {
    #[default]
    Unknow,
    Delimiter,
    Invalid,
    Found,
}
