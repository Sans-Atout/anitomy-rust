use crate::{
    elements::{Category, Elements},
    keyword::Manager,
    token::{main_token::Token, subtoken::SubTokenCategory},
};

pub fn parse_anime_title(tokens: &mut [Token], found_elements: &mut Elements, d: &[char]) {
    for token in tokens.iter_mut() {
        if token.contains_unknow() && !token.is_inside_delimiter() {
            println!("found token : {:?}", token);
            parse_particular_string_subtoken(token, found_elements, Category::AnimeTitle, d);
            return;
        }
    }

    let mut is_first_inside_delimiter_token = true;
    for token in tokens.iter_mut() {
        if token.contains_unknow() && token.is_inside_delimiter() {
            if is_first_inside_delimiter_token {
                is_first_inside_delimiter_token = false;
                continue;
            }
            parse_particular_string_subtoken(token, found_elements, Category::AnimeTitle, d);
            return;
        }
    }
}

pub fn parse_release_group(tokens: &mut [Token], found_elements: &mut Elements, d: &[char]) {
    if !found_elements.is_category_empty(Category::ReleaseGroup) {
        return;
    }
    for token in tokens.iter_mut() {
        if token.is_inside_delimiter() {
            if token.is_full_unknow() {
                for sub_token in token.sub_tokens() {
                    sub_token.category(SubTokenCategory::Found);
                }
                found_elements.add(Category::ReleaseGroup, &token.raw_token());
                return;
            }
            if token.contains_unknow() {
                parse_particular_string_subtoken(token, found_elements, Category::ReleaseGroup, d);
                return;
            }
        }
    }
}

pub fn parse_episode_title(tokens: &mut [Token], found_elements: &mut Elements, d: &[char]) {
    for token in tokens.iter_mut() {
        if token.contains_unknow() && !token.is_inside_delimiter() {
            parse_particular_string_subtoken(token, found_elements, Category::EpisodeTitle, d);
            return;
        }
    }
}

pub fn parse_particular_string_subtoken(
    token: &mut Token,
    e: &mut Elements,
    c: Category,
    d: &[char],
) {
    let all_subtoken = token.sub_tokens();
    let mut sub_token_id = 0;
    let mut string_to_categorise = String::default();
    while all_subtoken[sub_token_id].is_category(SubTokenCategory::Found) {
        sub_token_id += 1;
    }

    while sub_token_id < all_subtoken.len() {
        if all_subtoken[sub_token_id].is_category(SubTokenCategory::Found) {
            if !string_to_categorise.trim_matches(d).is_empty() {
                e.add(c, string_to_categorise.trim_matches(d));
            }
            string_to_categorise = String::default();
            break;
        }
        string_to_categorise = format!(
            "{} {}",
            string_to_categorise,
            all_subtoken[sub_token_id].value()
        );
        all_subtoken[sub_token_id].category(SubTokenCategory::Found);
        sub_token_id += 1;
    }
    if !string_to_categorise.is_empty() {
        e.add(c, string_to_categorise.trim_matches(d));
    }
}

pub fn parse_multiple_keyword(
    e: &mut Elements,
    keyword_manager: &Manager,
    left: &str,
    right: &str,
) -> bool {
    let mut tested = format!("{}.{}", left, right);
    if let Some(found) = keyword_manager.find(&tested.trim().to_uppercase()) {
        let category = found.get_category();
        let is_not =
            !category.is_searchable() || (category.is_singular() && !e.is_category_empty(category));
        if !is_not {
            e.add(category, tested.trim());
            return true;
        }
    }

    tested = format!("{} {}", left, right);
    if let Some(found) = keyword_manager.find(&tested.trim().to_uppercase()) {
        let category = found.get_category();
        let is_not =
            !category.is_searchable() || (category.is_singular() && !e.is_category_empty(category));
        if !is_not {
            e.add(category, tested.trim());
            return true;
        }
    }

    tested = format!("{}-{}", left, right);
    if let Some(found) = keyword_manager.find(&tested.trim().to_uppercase()) {
        let category = found.get_category();
        let is_not =
            !category.is_searchable() || (category.is_singular() && !e.is_category_empty(category));
        if !is_not {
            e.add(category, tested.trim());
            return true;
        }
    }

    tested = format!("{}_{}", left, right);
    if let Some(found) = keyword_manager.find(&tested.trim().to_uppercase()) {
        let category = found.get_category();
        let is_not =
            !category.is_searchable() || (category.is_singular() && !e.is_category_empty(category));
        if !is_not {
            e.add(category, tested.trim());
            return true;
        }
    }
    false
}
