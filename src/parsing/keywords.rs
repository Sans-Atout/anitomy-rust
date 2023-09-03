use crate::keyword::Keyword;
use crate::parsing::number;
use crate::traits::{ParsingNumber, EpisodeMatching, ExtendedString};
use crate::{chunk, elements};
use crate::{
    chunk::{Chunk, Status},
    elements::{Category, Elements},
    keyword::Manager,
};

pub fn parsing_keywords(elements: &mut Elements, tokens: &mut [Chunk]) {
    let manager = Manager::new();
    let mut chunk_id = 0;

    while chunk_id + 2 < tokens.len() {
        if tokens[chunk_id].is_status(Status::Found)
            || tokens[chunk_id + 1].is_status(Status::Found)
            || tokens[chunk_id + 2].is_status(Status::Found)
        {
            chunk_id += 1;
            continue;
        }

        if test_multiple_keywords(elements, tokens, &manager, &mut chunk_id) {
            continue;
        }

        chunk_id += 1;
    }

    for index in 0..tokens.len() {
        let tested = tokens[index].value();
        if tested.is_empty() || !tokens[index].is_status(crate::chunk::Status::Unknown) || (tested.is_digit() && tested.len() != 8 ){
            continue;
        }

        if tested.is_crc32() && elements.is_category_empty(Category::FileChecksum) {
            elements.add(Category::FileChecksum, &tested);
            tokens[index].found();
            continue;
        }

        if tested.is_resolution() && elements.is_category_empty(Category::VideoResolution) {
            elements.add(Category::VideoResolution, &tested);
            tokens[index].found();
            continue;
        }

        if let Some(found_keyword) = manager.find(&tested.to_uppercase()) {
            manage_single_keyword(elements, tokens, found_keyword, index);
        }
    }
}

fn test_multiple_keywords(e: &mut Elements, t: &mut [Chunk], m: &Manager, i: &mut usize) -> bool {
    if *i + 2 >= t.len() {
        return false;
    }

    let raw_format = format!(
        "{}{}{}",
        t[*i].value(),
        t[*i + 1].value(),
        t[*i + 2].value()
    );

    if raw_format.to_uppercase() == "E-AC" && *i + 4 < t.len() {
        let possible_eac3 = format!("{}{}{}", raw_format, t[*i + 3].value(), t[*i + 4].value());

        if possible_eac3.to_uppercase() == "E-AC-3" {
            e.add(Category::AudioTerm, &possible_eac3);
            t[*i].found();
            t[*i + 1].found();
            t[*i + 2].found();
            t[*i + 3].found();
            t[*i + 4].found();

            *i += 5;
            return true;
        }
    }

    if let Some(keyword) = m.find(&raw_format) {
        e.add(keyword.get_category(), &raw_format);

        t[*i].found();
        t[*i + 1].found();
        t[*i + 2].found();

        *i += 3;
        return true;
    }

    if t[*i + 1].is_status(Status::WeakDelimiter) {
        let space_test = format!("{} {}", t[*i].value(), t[*i + 2].value());
        if let Some(keyword) = m.find(&space_test) {
            e.add(keyword.get_category(), &space_test);
            t[*i].found();
            t[*i + 1].found();
            t[*i + 2].found();

            *i += 3;

            return true;
        }
    }

    false
}

fn manage_single_keyword(e: &mut Elements, t: &mut [Chunk], k: &Keyword, i: usize) -> bool {
    let category = k.get_category();
    let tested = t[i].value();
    if (category.is_singular() && !e.is_category_empty(category)) || !category.is_searchable() {
        return false;
    }
    if e.contains(category, &tested) {
        return false;
    }

    if category == Category::AnimeSeasonPrefix {
        if t.get(i - 2).is_some() {
            let ordinal_value = t[i-2].value();
            if !ordinal_value.ordinals_to_nb().is_empty() {
                e.add(Category::AnimeSeason, ordinal_value.ordinals_to_nb());
                e.add(Category::AnimeSeasonPrefix, &tested);
                t[i].found();
                t[i - 1].found();
                t[i - 2].found();
                return true;
            }
        }
        if let Some(possible_asn) = t.get(i + 2) {
            if possible_asn.value().is_digit() {
                e.add(Category::AnimeSeason, &possible_asn.value());
                e.add(Category::AnimeSeasonPrefix, &tested);
                t[i].found();
                t[i + 1].found();
                t[i + 2].found();
                return true;
            }
        }
    }

    if category == Category::EpisodePrefix {
        if let Some(possible_number) = t.get(i + 2) {
            if !possible_number.value().contains_digit() {
                return false;
            }
            if possible_number.value().is_partial_ep(e) {
                e.add(Category::EpisodePrefix, &t[i].value());
                t[i].found();
                t[i + 1].found();
                t[i + 2].found();
                return true;
            }
            if t.get(i + 4).is_some() {
                let episode_test = format!(
                    "{}{}{}",
                    t[i + 2].value(),
                    t[i + 3].value(),
                    t[i + 4].value()
                );
                if episode_test.is_multiple_ep( e) {
                    e.add(Category::EpisodePrefix, &t[i].value());
                    t[i].found();
                    t[i + 1].found();
                    t[i + 2].found();
                    t[i + 3].found();
                    t[i + 4].found();
                    return true;
                }

                if t.get(i + 6).is_some() && t[i + 2].value().is_digit()
                        && t[i + 3].is_status(Status::WeakDelimiter)
                        && t[i + 4].value() == "("
                        && t[i + 5].value().is_digit() && t[i + 6].value() == ")" {
                    e.add(Category::EpisodePrefix, &t[i].value());
                    e.add(Category::EpisodeNumber, &t[i + 1].value());
                    e.add(Category::EpisodeNumberAlt, &t[i + 5].value());
                    t[i].found();
                    t[i + 1].found();
                    t[i + 2].found();
                    t[i + 3].found();
                    t[i + 4].found();
                    t[i + 5].found();
                    t[i + 6].found();
                    return true;
                }
            }

            if possible_number.value().is_single_ep(e) {
                e.add(Category::EpisodePrefix, &t[i].value());
                t[i].found();
                t[i + 1].found();
                t[i + 2].found();
                return true;
            }

            if possible_number.value().is_japanese_ep(e) {
                e.add(Category::EpisodePrefix, &t[i].value());
                t[i].found();
                t[i + 1].found();
                t[i + 2].found();
                return true;
            }

            if possible_number.value().is_digit() {
                e.add(Category::EpisodePrefix, &t[i].value());
                e.add(Category::EpisodeNumber, &t[i + 2].value());
                t[i].found();
                t[i + 1].found();
                t[i + 2].found();
                return true;
            }
        }
    }

    if category == Category::ReleaseVersion {
        e.add(category, &tested.to_lowercase().replace('v', ""));
        t[i].found();
        return true;
    }

    if category == Category::VolumePrefix && t.get(i+2).is_some() {
        let potential_volume_number = t[i+2].value();
        if potential_volume_number.is_digit(){
            e.add(Category::VolumePrefix, &t[i].value());
            e.add(Category::VolumeNumber, &potential_volume_number);
            t[i].found();
            t[i+1].found();
            t[i+2].found();
            return true;
        }
    }

    if category == Category::AnimeType {
        e.add(Category::AnimeType, &tested);
        return true;
    }

    if category == Category::Language {
        if let Ok(languages) = e.find_all(Category::Language){
            if languages.iter().any(|l| l.value.normalize() == tested.normalize()){
                return false;
            }
            t[i].found();
            e.add(Category::Language,&tested);
            return true;
        }
    }

    if category != Category::Unknown {
        e.add(category, &tested);
        t[i].found();
        return true;
    }

    false
}

#[cfg(test)]
mod test {
    use crate::elements::Elements;
    use crate::keyword::Manager;
    use crate::split::split_raw_data;

    use super::test_multiple_keywords;

    const DELIMITER: [char; 4] = [' ', '-', '_', '.'];

    #[test]
    fn multiple_keyword() {
        let manager = Manager::new();
        let mut e = Elements::new();
        let mut id = 0;

        let mut chunks = split_raw_data("E-AC-3", &DELIMITER);

        assert!(test_multiple_keywords(
            &mut e,
            &mut chunks,
            &manager,
            &mut id
        ));

        id = 4;
        assert!(!test_multiple_keywords(
            &mut e,
            &mut chunks,
            &manager,
            &mut id
        ));

        id = 0;
        chunks = split_raw_data("5.1CH", &DELIMITER);
        assert!(test_multiple_keywords(
            &mut e,
            &mut chunks,
            &manager,
            &mut id
        ));

        id = 0;
        chunks = split_raw_data("Hello World", &DELIMITER);
        assert!(!test_multiple_keywords(
            &mut e,
            &mut chunks,
            &manager,
            &mut id
        ));
    }
}
