use std::vec;

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

impl Category {
    pub fn is_singular(&self) -> bool {
        let non_singular_categories = vec![
            Category::AnimeSeason,
            Category::AnimeType,
            Category::AudioTerm,
            Category::DeviceCompatibility,
            Category::EpisodeNumber,
            Category::Language,
            Category::Other,
            Category::ReleaseInformation,
            Category::Source,
            Category::VideoTerm,
        ];
        !non_singular_categories.contains(self)
    }

    pub fn is_searchable(&self) -> bool {
        let searchable = vec![
            Category::AnimeSeasonPrefix,
            Category::AnimeType,
            Category::AudioTerm,
            Category::DeviceCompatibility,
            Category::EpisodePrefix,
            Category::FileChecksum,
            Category::Language,
            Category::Other,
            Category::ReleaseGroup,
            Category::ReleaseInformation,
            Category::ReleaseVersion,
            Category::Source,
            Category::Subtitles,
            Category::VideoResolution,
            Category::VideoResolution,
            Category::VideoTerm,
            Category::VolumePrefix,
        ];
        searchable.contains(self)
    }
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

#[derive(Debug, Eq, Clone, Default)]
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

    pub fn is_category_empty(&self, c: Category) -> bool {
        self.count(c) == 0
    }
}

impl PartialEq for Elements {
    fn eq(&self, other: &Self) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }
        for e in self.elements.iter() {
            if !other.elements.contains(e) {
                return false;
            }
        }
        true
    }
}
