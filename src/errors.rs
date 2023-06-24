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
