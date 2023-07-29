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

    pub fn is_category(&self, c: SubTokenCategory) -> bool {
        self.category == c
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
