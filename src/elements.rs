use crate::errors::CategoryNotFound;
use error_stack::{Report, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    AnimeSeason,
    AnimeSeasonPrefix,
    AnimeTitle,
    AnimeType,
    AnimeYear,
    AudioTerm,
    DeviceCompatibility,
    EpisodeNumber,
    EpisodeNumberAlt,
    EpisodePrefix,
    EpisodeTitle,
    FileChecksum,
    FileExtension,
    FileName,
    Language,
    Other,
    ReleaseGroup,
    ReleaseInformation,
    ReleaseVersion,
    Source,
    Subtitles,
    VideoResolution,
    VideoTerm,
    VolumeNumber,
    VolumePrefix,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub category: Category,
    pub value: String,
}

impl Element {
    pub fn new(c: Category, v: &str) -> Element {
        Element {
            category: c,
            value: v.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Elements {
    elements: Vec<Element>,
}

impl Elements {
    pub fn new() -> Elements {
        Elements::default()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn add(&mut self, c: Category, v: &str) -> Elements {
        self.elements.push(Element::new(c, v));
        self.to_owned()
    }

    pub fn find(&self, c: Category) -> Result<Element, CategoryNotFound> {
        for e in self.elements.iter() {
            if c == e.category {
                return Ok(e.to_owned());
            }
        }
        Err(Report::new(CategoryNotFound))
    }

    pub fn find_all(&self, c: Category) -> Result<Vec<Element>, CategoryNotFound> {
        let mut all_elements: Vec<Element> = Vec::new();
        for e in self.elements.iter() {
            if c == e.category {
                all_elements.push(e.to_owned());
            }
        }
        if all_elements.is_empty() {
            return Err(Report::new(CategoryNotFound));
        }
        Ok(all_elements)
    }

    pub fn count(&self, c: Category) -> i32 {
        let mut count = 0;
        for e in self.elements.iter() {
            if c == e.category {
                count += 1;
            }
        }
        count
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
