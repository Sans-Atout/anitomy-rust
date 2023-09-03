use regex::Regex;

use crate::{
    chunk::{Chunk, Status},
    elements::{Category, Elements},
    keyword::Manager,
    split::split_type_and_ep,
    traits::{EpisodeMatching, ExtendedString, ParsingNumber},
};

const EP_DELIMITER: [char; 9] = ['-', '~', '&', '+', ' ', '.', '_', '-', 'x'];

pub fn parse_episode_number(d: &[char], c: &mut [Chunk], e: &mut Elements) -> bool {
    if d.iter().any(|d| EP_DELIMITER.contains(d)) {
        let mut index = 0;
        while let Some(data) = c.get(index + 2) {
            if !(c[index].is_status(Status::Unknown)
                && c[index + 1].is_status(Status::WeakDelimiter)
                && data.is_status(Status::Unknown))
            {
                index += 1;
                continue;
            }
            let tested = format!(
                "{}{}{}",
                c[index].value(),
                c[index + 1].value(),
                data.value()
            );
            if match_episode_string(d, &tested, e) {
                c[index].found();
                c[index + 1].found();
                c[index + 2].found();
                return true;
            }
            if c[index].value().is_digit()
                && c[index + 1].is_status(Status::WeakDelimiter)
                && c[index + 2].value() == "of"
                && c.get(index + 3)
                    .is_some_and(|c| c.is_status(Status::WeakDelimiter))
                && c.get(index + 4).is_some_and(|c| c.value().is_digit())
            {
                c[index].found();
                c[index + 1].found();
                c[index + 2].found();
                c[index + 3].found();
                c[index + 4].found();
                e.add(Category::EpisodeNumber, &c[index].value());
                return true;
            }
            index += 1;
        }
    }
    for chunk in c {
        if chunk.is_status(Status::Found) {
            continue;
        }
        if match_episode_string(d, &chunk.value(), e) {
            chunk.found();
        }
    }
    false
}

pub fn match_episode_string(d: &[char], s: &str, e: &mut Elements) -> bool {
    if s.is_multiple_ep(e) {
        return true;
    }

    // Season and episode
    if s.is_season_ep(e) {
        return true;
    }

    // Parse ep and type
    if s.match_episode_type_pattern(e, d) {
        return true;
    }

    // Parse single ep
    if s.is_single_ep(e) {
        return true;
    }

    // Episode like : 1.5 etc
    if s.is_fractal_ep(e) {
        return true;
    }

    // Episode 125a
    if s.is_partial_ep(e) {
        return true;
    }

    // Episode like #02v2
    if s.is_number_sign_pattern(e) {
        return true;
    }

    // Japanese counter like 750話
    if s.is_japanese_ep(e) {
        return true;
    }
    false
}

impl EpisodeMatching for str {
    fn is_fractal_ep(&self, e: &mut Elements) -> bool {
        let fractal_ep_regex = Regex::new(r"\d+\.5$").unwrap();
        if fractal_ep_regex.is_match(self) {
            e.add(Category::EpisodeNumber, self);
            return true;
        }
        false
    }

    fn is_japanese_ep(&self, e: &mut Elements) -> bool {
        let japanese_regex = Regex::new(r"(?P<episode_number>\d+)\u8A71$").unwrap();
        if let Some(captured) = japanese_regex.captures(self) {
            e.add(
                Category::EpisodeNumber,
                captured.name("episode_number").unwrap().as_str(),
            );
            return true;
        }
        false
    }

    fn is_partial_ep(&self, e: &mut Elements) -> bool {
        let mut non_number = false;
        let mut suffix_array = Vec::default();
        for test_char in self.chars() {
            if !non_number && test_char.is_ascii_digit() {
                continue;
            }
            non_number = true;
            suffix_array.push(test_char);
        }
        if let Some(first) = suffix_array.first() {
            if suffix_array.len() != 1 {
                return false;
            }
            if ['A', 'B', 'C'].contains(&first.to_ascii_uppercase()) {
                e.add(Category::EpisodeNumber, self);
                return true;
            }
        }
        false
    }

    fn is_season_ep(&self, e: &mut Elements) -> bool {
        let regex = Regex::new(r"^S?(?P<season_1>\d{1,2})(?:-S?(?P<season_2>\d{1,2}))?(?:x|[ ._-x]?E)(?P<ep_1>\d{1,4})(?:-E?(?P<ep_2>\d{1,4}))?(?:[vV](?P<version>\d))?$").unwrap();
        if let Some(captured) = regex.captures(self) {
            let first_season = match captured.name("season_1") {
                Some(data) => data.as_str(),
                None => "",
            };
            if first_season.parse::<i32>().unwrap_or(0) == 0 {
                return false;
            }
            e.add(Category::AnimeSeason, first_season);
            if let Some(season_2) = captured.name("season_2") {
                e.add(Category::AnimeSeason, season_2.as_str());
            }
            let ep_1 = captured.name("ep_1").unwrap().as_str();
            e.add(Category::EpisodeNumber, ep_1);
            if let Some(ep_2) = captured.name("ep_2") {
                e.add(Category::AnimeSeason, ep_2.as_str());
            }
            if let Some(version) = captured.name("version") {
                e.add(Category::AnimeSeason, version.as_str());
            }
            return true;
        }
        false
    }

    fn is_multiple_ep(&self, e: &mut Elements) -> bool {
        let regex = Regex::new(r"(?P<ep_1>\d{1,4})(?:[vV](?P<version_1>\d))?[-~&+](?P<ep_2>\d{1,4})(?:[vV](?P<version_2>\d))?$").unwrap();
        if let Some(captured) = regex.captures(self) {
            let ep_1 = captured.name("ep_1").unwrap().as_str();
            let ep_2 = captured.name("ep_2").unwrap().as_str();
            if ep_1.parse::<i32>().unwrap() > ep_2.parse::<i32>().unwrap() {
                return false;
            }
            e.add(Category::EpisodeNumber, ep_1);
            e.add(Category::EpisodeNumber, ep_2);

            if let Some(v1) = captured.name("version_1") {
                e.add(Category::ReleaseVersion, v1.as_str());
            }
            if let Some(v2) = captured.name("version_2") {
                e.add(Category::ReleaseVersion, v2.as_str());
            }
            return true;
        }
        false
    }

    fn is_single_ep(&self, e: &mut Elements) -> bool {
        let regex = Regex::new(r"(?P<episode>\d{1,4})[vV](?P<version>\d)$").unwrap();
        if let Some(captured) = regex.captures(self) {
            e.add(Category::EpisodeNumber, &captured["episode"]);
            e.add(Category::ReleaseVersion, &captured["version"]);
            return true;
        }
        false
    }

    fn match_episode_type_pattern(&self, e: &mut Elements, d: &[char]) -> bool {
        let (potential_key, data) = split_type_and_ep(self);
        let manager = Manager::new();
        let normalize_key = &potential_key.trim_matches(d);
        if let Some(keyword) = manager.find(&normalize_key.normalize()) {
            let category = keyword.get_category();
            if data.is_digit() {
                if category == Category::AnimeSeasonPrefix {
                    e.add(Category::AnimeSeason, &data);
                }
                if category == Category::EpisodePrefix || category == Category::AnimeType {
                    e.add(Category::EpisodeNumber, &data);
                }

                if category == Category::VolumePrefix {
                    e.add(Category::VolumeNumber, &data);
                }
                e.add(category, normalize_key);
                return true;
            }
            if match_episode_string(d, &data, e) {
                e.add(category, normalize_key);
                e.add(category, normalize_key);
            }
        }

        false
    }

    fn is_number_sign_pattern(&self, e: &mut Elements) -> bool {
        let regex =
            Regex::new(r"^#(?P<ep_1>\d{1,4})(?:[-~&+](?P<ep_2>\d{1,4}))?(?:[vV](?P<version>\d))?$")
                .unwrap();
        if let Some(captured) = regex.captures(self) {
            let ep_nb = captured.name("ep_1").unwrap();
            e.add(Category::EpisodeNumber, ep_nb.as_str());
            if let Some(ep_2) = captured.name("ep_2") {
                e.add(Category::EpisodeNumber, ep_2.as_str());
            }
            if let Some(version) = captured.name("version") {
                e.add(Category::EpisodeNumber, version.as_str());
            }
            return true;
        }
        false
    }
}
