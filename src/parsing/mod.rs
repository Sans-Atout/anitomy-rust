use crate::{
    elements::{Category, Elements},
    keyword::Manager,
    token::{main_token::Token, subtoken::SubTokenCategory},
};

use self::{
    number::{is_anime_year, is_crc32, is_digit, is_resolution, ordinals_to_nb},
    string::parse_multiple_keyword,
};

pub mod episode;
pub mod extensions;
pub mod number;
pub mod string;

pub fn parsing_single_token(elements: &mut Elements, token: &mut Token, manager: &Manager) {
    let sub_tokens = token.sub_tokens();
    for index in 0..sub_tokens.len() {
        let tested_value = sub_tokens[index].value();
        if tested_value.is_empty() || sub_tokens[index].is_category(SubTokenCategory::Found) {
            continue;
        }
        if is_digit(&tested_value) && tested_value.len() != 8 {
            if index + 1 < sub_tokens.len() {
                let left = sub_tokens[index].value();
                let right = sub_tokens[index + 1].value();
                if left == "5" && right == "1" {
                    sub_tokens[index].category(SubTokenCategory::Found);
                    sub_tokens[index + 1].category(SubTokenCategory::Found);
                    elements.add(Category::AudioTerm, "5.1");
                }
            }
            continue;
        }
        if is_crc32(&tested_value) && elements.is_category_empty(Category::FileChecksum) {
            elements.add(Category::FileChecksum, &tested_value);
            sub_tokens[index].category(SubTokenCategory::Found);
            continue;
        }
        if is_resolution(&tested_value) {
            elements.add(Category::VideoResolution, &tested_value);
            sub_tokens[index].category(SubTokenCategory::Found);
            continue;
        }
        if let Some(keyword) = manager.find(&tested_value.to_uppercase()) {
            let c = keyword.get_category();
            if (!c.is_searchable())
                || (c.is_singular() && !elements.is_category_empty(c))
                || elements.contains(c, &tested_value)
            {
                continue;
            }

            if tested_value.to_uppercase() == "E" && index + 2 < sub_tokens.len() {
                let supiscious_keyword = format!(
                    "{}-{}-{}",
                    tested_value,
                    sub_tokens[index + 1].value(),
                    sub_tokens[index + 2].value()
                );
                if supiscious_keyword.to_uppercase() == "E-AC-3" {
                    sub_tokens[index].category(SubTokenCategory::Found);
                    sub_tokens[index + 1].category(SubTokenCategory::Found);
                    sub_tokens[index + 2].category(SubTokenCategory::Found);
                    elements.add(Category::AudioTerm, &supiscious_keyword);
                    continue;
                }
            }
            if tested_value.to_uppercase() == "DTS" && index + 1 < sub_tokens.len() {
                let supiscious_keyword =
                    format!("{}-{}", tested_value, sub_tokens[index + 1].value());
                if supiscious_keyword.to_uppercase() == "DTS-ES" {
                    sub_tokens[index].category(SubTokenCategory::Found);
                    sub_tokens[index + 1].category(SubTokenCategory::Found);
                    elements.add(Category::AudioTerm, &supiscious_keyword);
                    continue;
                }
            }
            if c == Category::AnimeSeasonPrefix {
                elements.add(c, &tested_value);
                sub_tokens[index].category(SubTokenCategory::Found);
                if (index as i32 - 1) >= 0 {
                    let previous = sub_tokens[index - 1].value();
                    let p_saeson = ordinals_to_nb(&previous);
                    if !p_saeson.is_empty() {
                        elements.add(Category::AnimeSeason, p_saeson);
                        sub_tokens[index - 1].category(SubTokenCategory::Found);
                        continue;
                    }
                }
                if index + 1 < sub_tokens.len() {
                    let next = sub_tokens[index + 1].value();
                    if is_digit(&next) {
                        elements.add(Category::AnimeSeason, &next);
                        sub_tokens[index + 1].category(SubTokenCategory::Found);
                    }
                }
                continue;
            }

            if c == Category::EpisodePrefix {
                if index + 1 < sub_tokens.len() {
                    let next = sub_tokens[index + 1].value();
                    if is_digit(&next) {
                        elements.add(Category::EpisodeNumber, &next);
                        sub_tokens[index + 1].category(SubTokenCategory::Found);
                    }
                }
                elements.add(c, &tested_value);
                sub_tokens[index].category(SubTokenCategory::Found);
                continue;
            }

            if c == Category::ReleaseVersion {
                let release = sub_tokens[index].value().to_lowercase().replace('v', "");
                elements.add(Category::ReleaseVersion, &release);
                sub_tokens[index].category(SubTokenCategory::Found);
                continue;
            }
            if c == Category::VolumePrefix {
                if index + 1 < sub_tokens.len() {
                    let next = sub_tokens[index + 1].value();
                    if is_digit(&next) {
                        elements.add(Category::VolumeNumber, &next);
                        sub_tokens[index + 1].category(SubTokenCategory::Found);
                    }
                }
                elements.add(c, &tested_value);
                sub_tokens[index].category(SubTokenCategory::Found);
                continue;
            }
            if c == Category::AnimeType {
                elements.add(c, &tested_value);
                continue;
            }
            if c == Category::Language {
                let mut language_found = false;
                if let Ok(languages) = elements.find_all(Category::Language) {
                    for language in languages {
                        if language.value.to_uppercase() == tested_value.to_uppercase() {
                            language_found = true;
                            break;
                        }
                    }
                }
                if language_found {
                    continue;
                }
                elements.add(c, &tested_value);
                sub_tokens[index].category(SubTokenCategory::Found);
                continue;
            }
            if c != Category::Unknown {
                elements.add(c, &tested_value);
                sub_tokens[index].category(SubTokenCategory::Found);
                continue;
            }
        }

        if index + 1 < sub_tokens.len() {
            let left = sub_tokens[index].value();
            let right = sub_tokens[index + 1].value();
            if parse_multiple_keyword(elements, manager, &left, &right) {
                sub_tokens[index].category(SubTokenCategory::Found);
                sub_tokens[index + 1].category(SubTokenCategory::Found);
            }
        }
    }
}

pub fn parsing_keywords(elements: &mut Elements, tokens: &mut Vec<Token>) {
    let keyword_manager = Manager::new();

    for token in tokens {
        parsing_single_token(elements, token, &keyword_manager);
        if token.contains_unknow() && token.is_isolated_number() {
            let value = token.sub_tokens()[0].value();
            if is_anime_year(&value) {
                elements.add(Category::AnimeYear, &value);
                token.sub_tokens()[0].category(SubTokenCategory::Found);
            }
            if (value == "480" || value == "720" || value == "1080")
                && elements.is_category_empty(Category::VideoResolution)
            {
                elements.add(Category::VideoResolution, &value);
                token.sub_tokens()[0].category(SubTokenCategory::Found);
            }
        }
    }
}
