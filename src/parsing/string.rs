use regex::Regex;

use crate::{
    elements::{Category, Elements},
    keyword::Manager,
    token::{main_token::Token, subtoken::SubTokenCategory, self},
};

use super::number::is_digit;

pub fn parse_anime_title_old(tokens: &mut [Token], found_elements: &mut Elements, d: &[char]) {
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

pub fn parse_anime_title(tokens: &mut [Token], found_elements: &mut Elements, d: &[char]) {
    let mut tmp_index = 0;
    let mut anime_title = String::default();

    while tmp_index < tokens.len() {
        if tokens[tmp_index].contains_unknow() && !tokens[tmp_index].is_inside_delimiter() {
            anime_title = find_anime_title(tokens, tmp_index, d);
            println!("Anime Title : {}", anime_title);
            println!("##########################");
            println!("Token : {:?}", tokens);
            found_elements.add(Category::AnimeTitle, &anime_title);
            return; 
        }
        tmp_index += 1;
    }

    if anime_title == String::default(){
        tmp_index = 0;
        let mut is_potential_release_group = false;
        while tmp_index < tokens.len() {
            if tokens[tmp_index].contains_unknow() && tokens[tmp_index].is_inside_delimiter(){
                tmp_index += 1;
                if !is_potential_release_group{
                    is_potential_release_group = true;
                    continue;
                }
                anime_title = find_anime_title(tokens, tmp_index-1, d);
                found_elements.add(Category::AnimeTitle, &anime_title);
                return; 
            }
            tmp_index += 1;
        }
        //TODO
    }
}

fn find_anime_title(tokens : &mut [Token], starting_index : usize, d: &[char]) -> String{
    let mut token_index = starting_index; 

    println!("#############\nFind anime title\n!#############");
    let raw_token = tokens[token_index].raw_token();
    let subtoken = tokens[token_index].sub_tokens();
    let mut anime_title = String::default();
    let mut unknow_start = 0;

    while subtoken[unknow_start].is_category(SubTokenCategory::Found) {
        unknow_start += 1;
    }

    let mut tmp_index = unknow_start;
    while tmp_index < subtoken.len()  {
        if subtoken[tmp_index].is_category(SubTokenCategory::Found) {
            return anime_title.trim_matches(d).to_owned();
        }

        if is_digit(&subtoken[tmp_index].value()) && tmp_index + 1 < subtoken.len() && d.contains(&'.'){
            let second_token = subtoken[tmp_index+1].value();
            if is_digit(&second_token) {
                let regex = Regex::new(&format!(r"{}\.{}",&subtoken[tmp_index].value() , second_token)).unwrap();
                if regex.is_match(&raw_token){
                    anime_title = format!("{} {}.{}", anime_title, &subtoken[tmp_index].value(), &subtoken[tmp_index+1].value() )
                }
                subtoken[tmp_index].category(SubTokenCategory::Found);
                subtoken[tmp_index+1].category(SubTokenCategory::Found);
                tmp_index += 2;
                continue;    
            }
        }
        anime_title = format!("{} {}", anime_title, &subtoken[tmp_index].value());
        subtoken[tmp_index].category(SubTokenCategory::Found);
        tmp_index += 1;
    }

    token_index += 1; 

    while token_index < tokens.len() {
        let is_weak = tokens[token_index].is_weak();
        println!("----------------------\ntoken [{:?}]\n[{}][{}]\n----------------------\n",tokens[token_index],!tokens[token_index].contains_unknow(),(tokens[token_index].is_inside_delimiter() || !is_weak));
        if !tokens[token_index].contains_unknow() || (tokens[token_index].is_inside_delimiter() && !is_weak) {
            return anime_title.trim_matches(d).to_owned();
        }
        let vec_char = tokens[token_index-1].raw_token().chars().collect::<Vec<char>>();
        let last_char = vec_char.last().unwrap();
        let tmp_subtokens = tokens[token_index].sub_tokens();
        if tmp_subtokens[0].is_category(SubTokenCategory::Found) {
            return anime_title.trim_matches(d).to_owned();
        }

        if is_weak {
            if d.contains(last_char)  {
                anime_title = format!("{} ({}", anime_title, &tmp_subtokens[0].value());
            }else{
                anime_title = format!("{}({}", anime_title, &tmp_subtokens[0].value());
            }
        }else{
            anime_title = format!("{} {}", anime_title, &tmp_subtokens[0].value());
        }
        tmp_subtokens[0].category(SubTokenCategory::Found);

        for tmp_st in tmp_subtokens.iter_mut().skip(1)  {
            anime_title = format!("{} {}", anime_title, &tmp_st.value());
            tmp_st.category(SubTokenCategory::Found);
        }

        if is_weak {
            anime_title = format!("{})", anime_title);
        }
        token_index += 1; 
    }

    anime_title.trim_matches(d).to_owned()
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
