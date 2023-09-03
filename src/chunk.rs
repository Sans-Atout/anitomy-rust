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

impl ChunksManipulation for Vec<Chunk> {

    fn isolated_id(&self) -> Option<usize> {
        let mut is_unique_unk = true;
        let mut id = 0;
        let mut return_id = -1;
        while let Some(c) = self.get(id) {
            if c.is_status(Status::Unknown) {
                if !is_unique_unk {
                    return None;
                }
                is_unique_unk = false;
                return_id = id as i32;
            }
            id += 1;
        }
        usize::try_from(return_id).ok()
    }

    fn get_isolated_number(&self) -> Vec<usize> {
        let mut isolated_number_id = Vec::default();
        let mut starting_id = 0;
        let mut id = 0;
        let mut splitter_position: Vec<(usize, usize)> = Vec::default();
        let mut contain_unknown = false;

        while id < self.len() {
            if self[id].is_status(Status::StrongDelimiter) {
                if contain_unknown {
                    if !splitter_position.is_empty() {
                        contain_unknown = false;
                        for split_id in 0..splitter_position.len() {
                            let (_, starting) = splitter_position[split_id];
                            let ending = match splitter_position.get(split_id + 1) {
                                Some((s, _)) => s.to_owned(),
                                None => id,
                            };
                            for (in_split_id, chunk) in
                                self.iter().enumerate().take(ending).skip(starting)
                            {
                                if chunk.may_be_isolated() {
                                    isolated_number_id.push(in_split_id);
                                }
                            }
                        }
                        splitter_position = Vec::default();
                        starting_id = id;
                        id += 1;
                        continue;
                    }
                    for (usize_id, chunk) in self.iter().enumerate().take(id).skip(starting_id) {
                        if chunk.may_be_isolated() {
                            isolated_number_id.push(usize_id);
                        }
                    }
                }
                starting_id = id;
                id += 1;
                continue;
            }
            if self[id].is_status(Status::WeakDelimiter)
                && self
                    .get(id + 1)
                    .is_some_and(|c| c.is_status(Status::WeakDelimiter))
                && self
                    .get(id + 2)
                    .is_some_and(|c| c.is_status(Status::WeakDelimiter))
            {
                splitter_position.push((id, id + 2));
                id += 3;
                contain_unknown = false;
                continue;
            }
            if self[id].may_be_isolated() {
                contain_unknown = true;
            }
            id += 1;
        }
        if !splitter_position.is_empty() {
            for split_id in 0..splitter_position.len() {
                let (_, starting) = splitter_position[split_id];
                let ending = match splitter_position.get(split_id + 1) {
                    Some((s, _)) => s.to_owned(),
                    None => self.len(),
                };
                for (in_split_id, chunk) in self.iter().enumerate().take(ending).skip(starting) {
                    if chunk.may_be_isolated() {
                        isolated_number_id.push(in_split_id);
                    }
                }
            }
        }
        println!("hint : [{:?}]", splitter_position);
        isolated_number_id
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
