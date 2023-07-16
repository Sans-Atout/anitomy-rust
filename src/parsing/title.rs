use crate::{tokenizer::Token, elements::{Elements, Category}};

pub fn parse_anime_title(tokens : &mut [Token], found_elements : &mut Elements) {
    //let mut has_found_title = true;
}
pub fn parse_release_group(tokens : &mut [Token], found_elements : &mut Elements){
    if !found_elements.is_category_empty(Category::ReleaseGroup) {
        return;
    }
    for token in tokens.iter_mut() {
        if token.is_inside_delimiter() && token.contains_unknow() {
            let all_subtoken = token.sub_tokens();

            let mut sub_token_id = 0;
            let mut release_group = String::default();

            while all_subtoken[sub_token_id].is_found() {
                sub_token_id += 1;
            }

            while sub_token_id < all_subtoken.len() {
                if all_subtoken[sub_token_id].is_found() {
                    found_elements.add(Category::ReleaseGroup, release_group.trim());
                    release_group = String::default();
                    break;
                }
                release_group = format!("{} {}",release_group, all_subtoken[sub_token_id].value());
                all_subtoken[sub_token_id].category(crate::tokenizer::SubTokenCategory::Found);
                sub_token_id += 1;
            }

            if release_group != String::default() {
                found_elements.add(Category::ReleaseGroup, release_group.trim());
            }
        }
    }
}