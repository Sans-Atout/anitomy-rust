use crate::{tokenizer::Token, elements::{Elements, Category}};

pub fn parse_anime_title(tokens : &mut [Token], found_elements : &mut Elements) {
    for token in tokens.iter_mut() {
        if token.contains_unknow() && !token.is_inside_delimiter(){
            parse_particular_string_subtoken(token, found_elements,Category::AnimeTitle);
            return
        }
    }

    let mut is_first_inside_delimiter_token = true;
    for token in tokens.iter_mut() {
        if token.contains_unknow() && token.is_inside_delimiter(){
            if is_first_inside_delimiter_token{
                is_first_inside_delimiter_token = false;
                continue;
            }
            parse_particular_string_subtoken(token, found_elements,Category::AnimeTitle);
            return
        }
    }
}



pub fn parse_release_group(tokens : &mut [Token], found_elements : &mut Elements){
    if !found_elements.is_category_empty(Category::ReleaseGroup) {
        return;
    }
    for token in tokens.iter_mut() {
        if token.is_inside_delimiter() && token.contains_unknow() {
            parse_particular_string_subtoken(token, found_elements,Category::ReleaseGroup);
            return
        }
    }
}

pub fn parse_episode_title(tokens : &mut [Token], found_elements : &mut Elements) {
    for token in tokens.iter_mut() {
        if token.contains_unknow() && !token.is_inside_delimiter(){
            parse_particular_string_subtoken(token, found_elements,Category::EpisodeTitle);
            return
        }
    }

}

pub fn parse_particular_string_subtoken(token : &mut Token, e : &mut Elements, c : Category){
    let all_subtoken = token.sub_tokens();
    let mut sub_token_id = 0;
    let mut string_to_categorise = String::default();
    while all_subtoken[sub_token_id].is_found() {
        sub_token_id += 1;
    }

    while sub_token_id < all_subtoken.len() {
        if all_subtoken[sub_token_id].is_found() {
            e.add(c, string_to_categorise.trim());
            string_to_categorise = String::default();
            break;
        }
        string_to_categorise = format!("{} {}",string_to_categorise, all_subtoken[sub_token_id].value());
        all_subtoken[sub_token_id].category(crate::tokenizer::SubTokenCategory::Found);
        sub_token_id += 1;
    }
    if string_to_categorise != String::default() {
        e.add(c, string_to_categorise.trim());
    }
}