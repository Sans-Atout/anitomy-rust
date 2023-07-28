use elements::Elements;
use error_stack::{Report, Result};
use errors::ParsingError;
use parsing::{
    episode::parse_episode_number,
    extensions::{get_extension, remove_extension},
    number::is_anime_year,
    string::{parse_anime_title, parse_episode_title, parse_release_group},
};
use utils::remove_ignored_string;

use crate::{split::split_raw_data, token::token::Token};

pub mod elements;
pub mod errors;
pub mod keyword;
pub mod parsing;
pub mod split;
pub mod token;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Parser {
    file_name: String,
    ignored_string: Vec<String>,
    allowed_delimiters: Vec<char>,
    ep_number: bool,
    ep_title: bool,
    file_extension: bool,
    release_group: bool,
}

impl Parser {
    pub fn new(f_name: &str) -> Parser {
        Parser {
            file_name: f_name.to_string(),
            ignored_string: Vec::new(),
            allowed_delimiters: vec![' ', '_', '.', '-', '&', '+', ',', '|'],
            ep_number: true,
            ep_title: true,
            file_extension: true,
            release_group: true,
        }
    }

    pub fn ep_number(&mut self, need_to_parse: bool) -> Parser {
        self.ep_number = need_to_parse;
        self.to_owned()
    }

    pub fn ep_title(&mut self, need_to_parse: bool) -> Parser {
        self.ep_title = need_to_parse;
        self.to_owned()
    }

    pub fn file_extension(&mut self, need_to_parse: bool) -> Parser {
        self.file_extension = need_to_parse;
        self.to_owned()
    }

    pub fn release_group(&mut self, need_to_parse: bool) -> Parser {
        self.release_group = need_to_parse;
        self.to_owned()
    }

    pub fn file_name(&mut self, name: &str) -> Parser {
        self.file_name = name.to_string();
        self.to_owned()
    }

    pub fn ignored_string(&mut self, i: Vec<&str>) -> Parser {
        let mut ignored: Vec<String> = Vec::new();
        for s in i {
            ignored.push(s.to_string());
        }
        self.ignored_string = ignored;
        self.to_owned()
    }

    pub fn allowed_delimiters(&mut self, d: Vec<char>) -> Parser {
        self.allowed_delimiters = d;
        self.to_owned()
    }

    pub fn parse(&self) -> Result<Elements, ParsingError> {
        let mut _e = Elements::new().add(elements::Category::FileName, &self.file_name);

        // Remove file name extension
        let extension = get_extension(&self.file_name).unwrap_or_default();
        if !extension.is_empty() {
            _e.add(elements::Category::FileExtension, &extension);
        }

        let to_parse_str = remove_extension(&self.file_name);
        if to_parse_str.is_empty() {
            return Err(Report::new(ParsingError::StringIsEmpty)
                .attach_printable(format!("Can not parse file : {}", self.file_name)));
        }

        let raw_token = split_raw_data(
            &remove_ignored_string(&to_parse_str, &self.ignored_string),
            &self.allowed_delimiters,
        );

        let mut tokens_no_keyword: Vec<Token> = Vec::default();
        for mut t in raw_token {
            _e = t.parse(&mut _e);
            if t.is_isolated_number() {
                let token_value = t.sub_tokens().get(0).unwrap().value();
                if is_anime_year(&token_value) {
                    _e = t.keyword_found(elements::Category::AnimeYear, 0, &mut _e)
                }
                if (token_value == "480" || token_value == "720" || token_value == "1080")
                    && _e.is_category_empty(elements::Category::VideoResolution)
                {
                    _e = t.keyword_found(elements::Category::VideoResolution, 0, &mut _e)
                }
            }
            tokens_no_keyword.push(t);
        }

        if self.ep_number {
            parse_episode_number(&self.allowed_delimiters, &mut tokens_no_keyword, &mut _e)
        }
        parse_anime_title(&mut tokens_no_keyword, &mut _e);

        if self.release_group {
            parse_release_group(&mut tokens_no_keyword, &mut _e);
        }

        if self.ep_title {
            parse_episode_title(&mut tokens_no_keyword, &mut _e);
        }

        Ok(_e)
    }


}
