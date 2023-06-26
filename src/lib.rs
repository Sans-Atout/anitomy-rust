use elements::Elements;

pub mod elements;
pub mod errors;
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

    pub fn ignored_string(&mut self, i: Vec<String>) -> Parser {
        self.ignored_string = i;
        self.to_owned()
    }

    pub fn allowed_delimiters(&mut self, d: Vec<char>) -> Parser {
        self.allowed_delimiters = d;
        self.to_owned()
    }

    pub fn parse() -> Elements {
        let _e = Elements::new();
        todo!();
    }
}
