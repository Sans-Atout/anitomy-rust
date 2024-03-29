use regex::Regex;

use crate::{
    elements::{Category, Elements},
    keyword::Manager,
    split::split_type_and_ep,
    token::{main_token::Token, subtoken::SubTokenCategory},
};

use super::number::{contains_digit, is_digit};

pub fn parse_episode_number(
    delimiter: &Vec<char>,
    tokens_to_parse: &mut Vec<Token>,
    found_elements: &mut Elements,
) {
    for token in tokens_to_parse.iter_mut() {
        if !token.contains_unknow() {
            continue;
        }
        let subtokens = token.sub_tokens();
        for subtoken_id in 0..subtokens.len() {
            if subtokens[subtoken_id].is_category(SubTokenCategory::Found) {
                continue;
            }
            if subtokens[subtoken_id].value().is_empty() {
                subtokens[subtoken_id].category(SubTokenCategory::Found);
                continue;
            }
            if is_digit(&subtokens[subtoken_id].value())
                || !contains_digit(&subtokens[subtoken_id].value())
            {
                continue;
            }
            if parse_single_subtoken(delimiter, &subtokens[subtoken_id].value(), found_elements) {
                subtokens[subtoken_id].category(SubTokenCategory::Found);
            }
            // Episode like : 1.5 etc
            if match_fractal_episode(&subtokens[subtoken_id].value(), found_elements) {
                subtokens[subtoken_id].category(SubTokenCategory::Found);
                subtokens[subtoken_id + 1].category(SubTokenCategory::Found);
            }
        }
    }
    if found_elements.is_category_empty(Category::EpisodeNumber) {
        for token in tokens_to_parse.iter_mut() {
            if !token.contains_unknow() {
                continue;
            }
            let raw_data = token.raw_token();
            let sub_tokens = token.sub_tokens();
            for index in 0..sub_tokens.len() {
                let tested_value = sub_tokens[index].value();
                if is_digit(&tested_value) {
                    if let Some(next_value) = sub_tokens.get(index + 1) {
                        if is_digit(&next_value.value()) {
                            let right = next_value.value().parse::<i32>().unwrap();
                            let left = tested_value.parse::<i32>().unwrap();
                            let fractal_regex =
                                Regex::new(&format!(r"{}\.{}", tested_value, right)).unwrap();

                            let fractal_match = fractal_regex.is_match(&raw_data);
                            if right == 5 && fractal_match {
                                sub_tokens[index].category(SubTokenCategory::Found);
                                sub_tokens[index + 1].category(SubTokenCategory::Found);
                                found_elements
                                    .add(Category::EpisodeNumber, &format!("{}.5", tested_value));
                                return;
                            }
                            if fractal_match {
                                continue;
                            }
                            if left < right {
                                sub_tokens[index].category(SubTokenCategory::Found);
                                sub_tokens[index + 1].category(SubTokenCategory::Found);
                                found_elements
                                    .add(Category::EpisodeNumber, &sub_tokens[index].value());
                                found_elements
                                    .add(Category::EpisodeNumber, &sub_tokens[index + 1].value());
                                return;
                            }
                        }
                    }
                    if let Some(sub_token) = sub_tokens.get(index + 2) {
                        if is_digit(&sub_token.value()) {
                            let middle = sub_tokens[index + 1].value();
                            let right = sub_token.value().parse::<i32>().unwrap();
                            let left = tested_value.parse::<i32>().unwrap();
                            let p_delimiter = middle.chars().next().unwrap();
                            if middle == "of" && left < right {
                                sub_tokens[index].category(SubTokenCategory::Found);
                                sub_tokens[index + 1].category(SubTokenCategory::Found);
                                sub_tokens[index + 2].category(SubTokenCategory::Found);
                                found_elements
                                    .add(Category::EpisodeNumber, &sub_tokens[index].value());
                                return;
                            }

                            if delimiter.contains(&p_delimiter) && middle.len() == 1 && left < right
                            {
                                sub_tokens[index].category(SubTokenCategory::Found);
                                sub_tokens[index + 1].category(SubTokenCategory::Found);
                                found_elements
                                    .add(Category::EpisodeNumber, &sub_tokens[index].value());
                                found_elements
                                    .add(Category::EpisodeNumber, &sub_tokens[index + 1].value());
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    if found_elements.is_category_empty(Category::EpisodeNumber) {
        for token_index in 0..tokens_to_parse.len() {
            if let Some(tmp_token) = tokens_to_parse.get_mut(token_index) {
                if !tmp_token.contains_unknow() {
                    continue;
                }
                let raw_token = tmp_token.raw_token();
                let sub_token = tmp_token.sub_tokens();
                let mut subtoken_index = 0;
                while subtoken_index < sub_token.len() {
                    if let Some(tested_subtoken) = sub_token.get(subtoken_index) {
                        subtoken_index += 1;
                        if tested_subtoken.is_category(SubTokenCategory::Found)
                            || !is_digit(&tested_subtoken.value())
                        {
                            continue;
                        }
                        if let Some(next_token) = sub_token.get(subtoken_index) {
                            if is_digit(&next_token.value())
                                && !next_token.is_category(SubTokenCategory::Found)
                            {
                                let fractal_regex_string = format!(
                                    r"{}\.{}",
                                    &tested_subtoken.value(),
                                    &next_token.value()
                                );
                                let fractal_regex = Regex::new(&fractal_regex_string).unwrap();
                                if fractal_regex.is_match(&raw_token) {
                                    subtoken_index += 1;
                                    continue;
                                }
                            }
                        }
                        found_elements.add(Category::EpisodeNumber, &tested_subtoken.value());
                        sub_token[subtoken_index - 1].category(SubTokenCategory::Found);
                        return;
                    }
                }
            }
        }
    }
}

pub fn parse_single_subtoken(
    delimiter: &Vec<char>,
    string_to_parse: &str,
    found_elements: &mut Elements,
) -> bool {
    if match_multiple_ep(string_to_parse, found_elements) {
        return true;
    }

    // Saeson and episode
    if match_season_ep_patern(string_to_parse, found_elements) {
        return true;
    }

    // Parse ep and type
    if match_type_episode(string_to_parse, found_elements, delimiter) {
        return true;
    }

    // Parse single ep
    if parse_single_ep(string_to_parse, found_elements) {
        return true;
    }

    // Episode like : 1.5 etc
    if match_fractal_episode(string_to_parse, found_elements) {
        return true;
    }

    // Episode 125a
    if match_partial_episode_pattern(string_to_parse, found_elements) {
        return true;
    }

    // Episode like #02v2
    if match_number_sign_patern(string_to_parse, found_elements) {
        return true;
    }

    // Japanese counter like 750話
    if match_japanese_counter(string_to_parse, found_elements) {
        return true;
    }
    false
}

pub fn parse_single_ep(tested_string: &str, found_elements: &mut Elements) -> bool {
    let single_ep_regex = Regex::new(r"(?P<episode>\d{1,4})[vV](?P<version>\d)$").unwrap();
    if single_ep_regex.is_match(tested_string) {
        let single_ep_captures = single_ep_regex.captures(tested_string).unwrap();
        found_elements.add(Category::EpisodeNumber, &single_ep_captures["episode"]);
        found_elements.add(Category::ReleaseVersion, &single_ep_captures["version"]);
        return true;
    }
    false
}

pub fn match_multiple_ep(tested_string: &str, found_elements: &mut Elements) -> bool {
    let multiple_ep_regex = Regex::new(r"(?P<ep_1>\d{1,4})(?:[vV](?P<version_1>\d))?[-~&+](?P<ep_2>\d{1,4})(?:[vV](?P<version_2>\d))?$").unwrap();
    if multiple_ep_regex.is_match(tested_string) {
        let multiple_ep_captures = multiple_ep_regex.captures(tested_string).unwrap();
        let ep_1 = multiple_ep_captures.name("ep_1").unwrap().as_str();
        let ep_2 = multiple_ep_captures.name("ep_2").unwrap().as_str();
        if ep_1.parse::<i32>().unwrap() > ep_2.parse::<i32>().unwrap() {
            return false;
        }
        found_elements.add(Category::EpisodeNumber, ep_1);
        found_elements.add(Category::EpisodeNumberAlt, ep_2);

        if let Some(v1) = multiple_ep_captures.name("version_1") {
            found_elements.add(Category::ReleaseVersion, v1.as_str());
        }
        if let Some(v2) = multiple_ep_captures.name("version_2") {
            found_elements.add(Category::ReleaseVersion, v2.as_str());
        }
        return true;
    }
    false
}

pub fn match_season_ep_patern(tested_string: &str, found_elements: &mut Elements) -> bool {
    let season_episode_regex = Regex::new(r"S?(?P<season_1>\d{1,2})(?:-S?(?P<season_2>\d{1,2}))?(?:x|[ ._-x]?E)(?P<ep_1>\d{1,4})(?:-E?(?P<ep_2>\d{1,4}))?(?:[vV](?P<version>\d))?$").unwrap();
    if season_episode_regex.is_match(tested_string) {
        let season_ep_captures = season_episode_regex.captures(tested_string).unwrap();
        let season_1 = season_ep_captures.name("season_1").unwrap().as_str();
        if season_1.parse::<i32>().unwrap() == 0 {
            return false;
        }
        found_elements.add(Category::AnimeSeason, season_1);
        if let Some(season_2) = season_ep_captures.name("season_2") {
            found_elements.add(Category::AnimeSeason, season_2.as_str());
        }
        let ep_1 = season_ep_captures.name("ep_1").unwrap().as_str();
        found_elements.add(Category::EpisodeNumber, ep_1);
        if let Some(ep_2) = season_ep_captures.name("ep_2") {
            found_elements.add(Category::AnimeSeason, ep_2.as_str());
        }
        if let Some(version) = season_ep_captures.name("version") {
            found_elements.add(Category::AnimeSeason, version.as_str());
        }
        return true;
    }
    false
}

pub fn match_type_episode(
    tested_string: &str,
    found_elements: &mut Elements,
    delimiter: &Vec<char>,
) -> bool {
    let (potential_keyword, data_to_parse) = split_type_and_ep(tested_string);
    let keyword_manager = Manager::new();
    let trim_keyword = potential_keyword.trim_matches(delimiter.as_slice());
    if let Some(keyword) = keyword_manager.find(&trim_keyword.to_uppercase()) {
        let tmp_c = keyword.get_category();
        found_elements.add(tmp_c, trim_keyword);
        if is_digit(&data_to_parse) {
            if tmp_c == Category::AnimeSeasonPrefix {
                found_elements.add(Category::AnimeSeason, &data_to_parse);
            }
            if tmp_c == Category::EpisodePrefix || tmp_c == Category::AnimeType {
                found_elements.add(Category::EpisodeNumber, &data_to_parse);
            }
            if tmp_c == Category::VolumePrefix {
                found_elements.add(Category::VolumeNumber, &data_to_parse);
            }
            return true;
        }
        parse_single_subtoken(delimiter, &data_to_parse, found_elements);
        return true;
    }
    false
}

pub fn match_fractal_episode(tested_string: &str, found_elements: &mut Elements) -> bool {
    let multiple_ep_regex = Regex::new(r"\d+\.5$").unwrap();
    if multiple_ep_regex.is_match(tested_string) {
        found_elements.add(Category::EpisodeNumber, tested_string);
        return true;
    }
    false
}

pub fn match_japanese_counter(tested_string: &str, found_elements: &mut Elements) -> bool {
    let japanese_regex = Regex::new(r"(?P<episode_number>\d+)\u8A71$").unwrap();
    if japanese_regex.is_match(tested_string) {
        let episode_number = japanese_regex
            .captures(tested_string)
            .unwrap()
            .name("episode_number")
            .unwrap()
            .as_str();
        found_elements.add(Category::EpisodeNumber, episode_number);
        return true;
    }
    false
}

pub fn match_number_sign_patern(tested_string: &str, found_elements: &mut Elements) -> bool {
    let number_sign_regex =
        Regex::new(r"^#(?P<ep_1>\d{1,4})(?:[-~&+](?P<ep_2>\d{1,4}))?(?:[vV](?P<version>\d))?$")
            .unwrap();
    if number_sign_regex.is_match(tested_string) {
        let captured_data = number_sign_regex.captures(tested_string).unwrap();
        let ep_nb = captured_data.name("ep_1").unwrap();
        found_elements.add(Category::EpisodeNumber, ep_nb.as_str());
        if let Some(ep_2) = captured_data.name("ep_2") {
            found_elements.add(Category::EpisodeNumber, ep_2.as_str());
        }
        if let Some(version) = captured_data.name("version") {
            found_elements.add(Category::EpisodeNumber, version.as_str());
        }
        return true;
    }
    false
}

pub fn match_partial_episode_pattern(tested_string: &str, found_elements: &mut Elements) -> bool {
    let mut non_number = false;
    let mut suffix_array = Vec::default();
    for test_char in tested_string.chars() {
        if !non_number && test_char.is_ascii_digit() {
            continue;
        }
        non_number = true;
        suffix_array.push(test_char);
    }
    if suffix_array.len() == 1
        && vec!['A', 'B', 'C'].contains(&suffix_array.first().unwrap().to_ascii_uppercase())
    {
        found_elements.add(Category::EpisodeNumber, tested_string);
        return true;
    }

    false
}
