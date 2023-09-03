use crate::{chunk::Chunk};

pub const OPENING_DELIMITER : [char; 7] = ['[', '(', '{', '\u{300C}', '\u{300E}', '\u{3010}', '\u{FF08}'];
pub const CLOSING_DELIMITER : [char; 7] = [']', ')', '}', '\u{300D}', '\u{300F}', '\u{3011}', '\u{FF09}'];

pub fn split_type_and_ep(to_parse: &str) -> (String, String) {
    let mut keyword_char: Vec<char> = Vec::default();
    let mut to_parse_char: Vec<char> = Vec::default();
    let mut has_pass_digit = false;

    for s_char in to_parse.chars() {
        if s_char.is_ascii_digit() {
            has_pass_digit = true;
        }

        if has_pass_digit {
            to_parse_char.push(s_char);
        } else {
            keyword_char.push(s_char);
        }
    }

    let keyword = keyword_char.iter().cloned().collect::<String>();
    let to_check = to_parse_char.iter().cloned().collect::<String>();
    (keyword, to_check)
}

/// New version of raw data splitting function
pub fn split_raw_data(raw_data: &str, delimiter: &[char]) -> Vec<Chunk> {
    let mut token: Vec<Chunk> = Vec::default();

    let mut _important_delimiter_end: char = char::default();

    let raw_chars: Vec<char> = raw_data.chars().collect();
    let mut depth : i16 = 0;
    let mut tmp_word : Vec<char> = Vec::default();

    for c in raw_chars {
        if OPENING_DELIMITER.contains(&c) {
            tmp_word =  reset_tmp_word(&mut token, &tmp_word, &depth);
            depth += 1;
            if let Ok(chunk) = Chunk::new(&format!("{c}"), depth, crate::chunk::Status::StrongDelimiter){
                token.push(chunk)
            }
            continue;
        }
        if CLOSING_DELIMITER.contains(&c) {
            tmp_word =  reset_tmp_word(&mut token, &tmp_word, &depth);
            if let Ok(chunk) = Chunk::new(&format!("{c}"), depth, crate::chunk::Status::StrongDelimiter){
                token.push(chunk)
            }
            if depth > 0 {
                depth -= 1;
            }
            continue;
        }
        if delimiter.contains(&c){
            tmp_word =  reset_tmp_word(&mut token, &tmp_word, &depth);
            if let Ok(chunk) = Chunk::new(&format!("{c}"), depth, crate::chunk::Status::WeakDelimiter){
                token.push(chunk)
            }
            continue;
        }
        tmp_word.push(c);
    }

    if !tmp_word.is_empty() {
        if let Ok(chunk) = Chunk::new(&tmp_word.iter().collect::<String>(), depth, crate::chunk::Status::Unknown) {
            token.push(chunk);
        }
    
    }

    token
}

fn reset_tmp_word(token : &mut Vec<Chunk>, word : &[char], depth : &i16) -> Vec<char>{
    if word.is_empty(){
        return Vec::default();
    }
    if let Ok(chunk) = Chunk::new(&word.iter().collect::<String>(), *depth, crate::chunk::Status::Unknown) {
        token.push(chunk);
    }
    Vec::default()
}
