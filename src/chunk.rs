use error_stack::{Report, Result, ResultExt};
use std::fmt::{Debug, Display};

use crate::{
    errors::ParsingError,
    traits::{ChunksManipulation, ParsingNumber},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Chunk {
    value: String,
    depth: i16,
    status: Status,
}

impl Chunk {
    pub fn new(raw: &str, depth: i16, chunk_status: Status) -> Result<Chunk, ParsingError> {
        if raw.is_empty() {
            return Err(Report::new(ParsingError::StringIsEmpty))
                .attach_printable(format!("string : [{raw}] is empty"));
        }

        Ok(Chunk {
            value: raw.to_string(),
            depth,
            status: chunk_status,
        })
    }

    pub fn is_status(&self, s: Status) -> bool {
        self.status == s
    }

    pub fn value(&self) -> String {
        self.value.to_owned()
    }

    pub fn depth(&self) -> i16 {
        self.depth
    }

    pub fn found(&mut self) {
        self.status = Status::Found;
    }

    pub fn may_be_isolated(&self) -> bool {
        self.status == Status::Unknown && self.value.is_digit()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    Unknown,
    Isolated,
    Found,
    StrongDelimiter,
    WeakDelimiter,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Unknown => f.write_str("unknown"),
            Status::Isolated => f.write_str("isolated"),
            Status::Found => f.write_str("found"),
            Status::StrongDelimiter => f.write_str("strong delimiter"),
            Status::WeakDelimiter => f.write_str("weak delimiter"),
        }
    }
}
#[cfg(test)]
pub mod test {
    use super::Chunk;
    use super::Status;
    use std::fmt::{Debug, Display};

    fn entity_test<
        T: Sized + Send + Sync + Unpin + Debug + Clone + Copy + PartialEq + Eq + Display,
    >() {
    }

    #[test]
    fn chunk() {
        let chunk_wrap = Chunk::new("New chunk", 0, super::Status::Isolated);
        assert!(chunk_wrap.is_ok());
        let mut chunk = chunk_wrap.unwrap();
        assert_eq!(chunk.value(), "New chunk");
        assert_eq!(chunk.depth(), 0);
        assert!(chunk.is_status(Status::Isolated));
        assert!(!chunk.is_status(Status::Found));
        chunk.found();
        assert!(chunk.is_status(Status::Found));
    }

    #[test]
    #[should_panic]
    fn chunk_panic() {
        Chunk::new("", 0, super::Status::Unknown).unwrap();
    }

    #[test]
    fn status_entity() {
        assert_eq!(Status::default(), Status::Unknown);
        entity_test::<Status>();
    }
}
