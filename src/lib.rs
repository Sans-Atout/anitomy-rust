use elements::Elements;
use error_stack::{Report, Result};
use errors::ParsingError;
use tokenizer::{tokenize, Token};
use utils::{get_extension, remove_extension, remove_ignored_string};

pub mod elements;
pub mod errors;
pub mod keyword;
pub mod parsing;
pub mod tokenizer;
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
            allowed_delimiters: vec![' ', '_', '.', '&', '+', ',', '|'],
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

        let raw_token = tokenize(
            &remove_ignored_string(&to_parse_str, self.ignored_string.to_owned()),
            &self.allowed_delimiters,
        );
        let mut tokens_no_keyword : Vec<Token> = Vec::default();

        for mut t in raw_token {
            _e = t.parse(&mut _e);
            tokens_no_keyword.push(t);
        }
        Ok(_e)
    }
}
