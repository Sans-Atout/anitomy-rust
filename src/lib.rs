use elements::Elements;
use error_stack::{Report, Result};
use errors::ParsingError;
use parsing::{
    extensions::{get_extension, remove_extension},
    string::{parse_anime_title, parse_episode_title, parse_release_group},
};
use split::split_raw_data;

use crate::{
    parsing::{
        episode::parse_episode_number, keywords::parsing_keywords, number::parsing_isolated_number,
    },
    traits::{ChunksManipulation, ExtendedString},
};

pub mod chunk;
pub mod elements;
pub mod errors;
mod keyword;
mod parsing;
mod split;
mod traits;
mod utils;

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
        let mut found = Elements::new().add(elements::Category::FileName, &self.file_name);

        // Remove file name extension
        let extension = get_extension(&self.file_name).unwrap_or_default();
        if !extension.is_empty() {
            found.add(elements::Category::FileExtension, &extension);
        }

        let mut string_to_parse = remove_extension(&self.file_name);

        if string_to_parse.is_empty() {
            return Err(Report::new(ParsingError::StringIsEmpty)
                .attach_printable(format!("Can not parse file : {}", self.file_name)));
        }
        string_to_parse = string_to_parse.remove_ignored(&self.ignored_string);
        let mut chunks = split_raw_data(&string_to_parse, &self.allowed_delimiters);
        parsing_keywords(&mut found, &mut chunks);

        if self.ep_number {
            parse_episode_number(&self.allowed_delimiters, &mut chunks, &mut found);
        }

        parsing_isolated_number(&mut found, &chunks.get_isolated_number(), &mut chunks);
        parse_anime_title(&mut chunks, &mut found, &self.allowed_delimiters);

        if self.release_group {
            parse_release_group(&mut chunks, &mut found, &self.allowed_delimiters);
        }

        if self.ep_title {
            parse_episode_title(&mut chunks, &mut found, &self.allowed_delimiters);
        }

        Ok(found)
    }
}
