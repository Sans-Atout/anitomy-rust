use crate::token::main_token::Token;

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

pub fn split_raw_data(raw_data: &str, delimiter: &[char]) -> Vec<Token> {
    let mut raw_tokens: Vec<Token> = Vec::default();
    let raw_as_chars: Vec<char> = raw_data.chars().collect();

    let mut tmp_raw_token: Vec<char> = Vec::default();

    for raw_char in raw_as_chars {
        match raw_char {
            '[' | '(' | '{' | '\u{300C}' | '\u{300E}' | '\u{3011}' | '\u{FF08}' => {
                if !tmp_raw_token.is_empty() {
                    let token_str = tmp_raw_token.iter().collect::<String>();
                    raw_tokens.push(Token::new(&token_str, delimiter, false, false));
                }
                tmp_raw_token = vec![];
            }
            ']' | ')' | '}' | '\u{300D}' | '\u{300F}' | '\u{3010}' | '\u{FF09}' => {
                if !tmp_raw_token.is_empty() {
                    let token_str = tmp_raw_token.iter().collect::<String>();
                    raw_tokens.push(Token::new(&token_str, delimiter, true, raw_char == ')'));
                }
                tmp_raw_token = vec![];
            }
            _ => {
                tmp_raw_token.push(raw_char);
            }
        }
    }
    if !tmp_raw_token.is_empty() {
        let token_str = tmp_raw_token.iter().cloned().collect::<String>();
        raw_tokens.push(Token::new(&token_str, delimiter, false, false));
    }
    raw_tokens
}

pub fn split_token(raw_token: &str, delimiter: &[char]) -> Vec<String> {
    let mut tokenized_token : Vec<String> = Vec::default();
    let token_as_char: Vec<char> = raw_token.trim_matches(delimiter).chars().collect();

    let mut token_index = 0;
    let mut tmp_sub_token: Vec<char> = Vec::default();

    while token_index < token_as_char.len() {
        
        let token_char = token_as_char[token_index];
        token_index += 1;

        let is_delimiter = delimiter.contains(&token_char);
        
        if is_delimiter  {
            if tmp_sub_token.is_empty() {
                continue;
            }
            let sub_token = tmp_sub_token.iter().collect::<String>();
            tokenized_token.push(sub_token);
            tmp_sub_token = Vec::default();

            if token_index + 1 >= token_as_char.len() {
                continue;
            }

            if delimiter.contains(&token_as_char[token_index]) && delimiter.contains(&token_as_char[token_index+1]){
                tokenized_token.push(token_as_char[token_index].to_string());
                token_index += 1;
            }
            continue;    
        }
        tmp_sub_token.push(token_char);
    }

    if !tmp_sub_token.is_empty() {
        let sub_token = tmp_sub_token.iter().collect::<String>();
        tokenized_token.push(sub_token);
    }
    tokenized_token
}