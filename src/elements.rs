use crate::errors::CategoryNotFound;
use error_stack::{Report, Result};

const SEARCHABLE_CATEGORIES: [Category; 17] = [
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

const NON_SINGULAR_CATEGORIES: [Category; 10] = [
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
        !NON_SINGULAR_CATEGORIES.contains(self)
    }

    pub fn is_searchable(&self) -> bool {
        SEARCHABLE_CATEGORIES.contains(self)
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

    pub fn contains(&self, c: Category, v: &str) -> bool {
        self.elements.contains(&Element::new(c, v))
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
                println!("{:?}", e);
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::elements::{Category, Element, Elements};

    #[test]
    fn elements_eq() {
        let e_1 = Elements::new().add(Category::AnimeSeason, "01");
        let e_2 = Elements::new().add(Category::AnimeSeason, "01");
        let e_3 = Elements::new()
            .add(Category::AnimeSeason, "02")
            .add(Category::AnimeSeason, "01");
        let e_4 = Elements::new().add(Category::EpisodeNumber, "01");
        assert_eq!(e_1, e_2);
        assert_eq!(e_1.size(), 1);
        assert_ne!(e_1, e_3);
        assert_ne!(e_2, e_3);
        assert_ne!(e_2, e_4);
    }

    #[test]
    fn element_eq() {
        let e1 = Element::new(Category::AnimeSeason, "01");
        let e2 = Element::new(Category::AnimeSeason, "01");
        let e3 = Element::new(Category::AnimeSeason, "02");
        let e4 = Element::new(Category::EpisodeNumber, "01");
        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
        assert_ne!(e2, e3);
        assert_ne!(e2, e4);
    }

    #[test]
    fn count() {
        let elements = Elements::new()
            .add(Category::AnimeSeason, "01")
            .add(Category::AnimeSeasonPrefix, "S")
            .add(Category::AnimeSeason, "02");
        assert_eq!(elements.count(Category::AnimeSeason), 2);
    }

    #[test]
    fn find_all() {
        let vec_e = vec![
            Element::new(Category::AnimeSeason, "01"),
            Element::new(Category::AnimeSeason, "02"),
        ];
        let elements = Elements::new()
            .add(Category::AnimeSeason, "01")
            .add(Category::AnimeSeasonPrefix, "S")
            .add(Category::AnimeSeason, "02");
        let all_elements = elements.find_all(Category::AnimeSeason);
        assert!(all_elements.is_ok());
        let a_e = all_elements.unwrap();
        assert_eq!(a_e, vec_e);
    }

    #[test]
    fn find_all_not_found() {
        let element = Elements::new();
        assert!(element.find_all(Category::AnimeSeason).is_err());
    }

    #[test]
    fn size() {
        let elements = Elements::new()
            .add(Category::AnimeSeason, "01")
            .add(Category::AnimeSeasonPrefix, "S")
            .add(Category::AnimeSeason, "02");
        assert_eq!(elements.size(), 3);
    }

    #[test]
    fn is_empty() {
        let mut elements = Elements::new();
        assert!(elements.is_empty());
        elements = elements
            .add(Category::AnimeSeason, "01")
            .add(Category::AnimeSeasonPrefix, "S")
            .add(Category::AnimeSeason, "02");
        assert!(!elements.is_empty());
    }

    #[test]
    fn empty_category() {
        let mut e_ = Elements::new().add(Category::AnimeSeason, "01");
        assert!(!e_.is_category_empty(Category::AnimeSeason));
        assert!(e_.is_category_empty(Category::AnimeTitle));
        e_.add(Category::AnimeTitle, "My Super Anime");
        assert!(!e_.is_category_empty(Category::AnimeTitle));
    }

    #[test]
    fn is_equal() {
        let e_1 = Elements::new()
            .add(Category::AnimeSeason, "01")
            .add(Category::EpisodeNumber, "01");
        let e_2 = Elements::new()
            .add(Category::EpisodeNumber, "01")
            .add(Category::AnimeSeason, "01");
        let e_3 = Elements::new()
            .add(Category::EpisodeNumber, "01")
            .add(Category::AnimeSeason, "01")
            .add(Category::AnimeYear, "2009");
        assert_eq!(e_1, e_2);
        assert_ne!(e_1, e_3);
    }
}
