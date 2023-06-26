use error_stack::Context;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CategoryNotFound;
impl Context for CategoryNotFound {}

impl fmt::Display for CategoryNotFound {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Category Not Found : given category was not found")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingError {
    StringIsEmpty,
    NoExtension
}
impl Context for ParsingError{}
impl fmt::Display for ParsingError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::StringIsEmpty => fmt.write_str("Parsing Error : nothing to parse"),
            ParsingError::NoExtension => fmt.write_str("Parsing Error : no extension"),
        }
        
    }
}