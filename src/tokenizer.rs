pub fn tokenize(string_to_tokenize: &str) -> Vec<String> {
    let mut all_token: Vec<String> = vec![];
    let token_char_vec: Vec<char> = string_to_tokenize.chars().collect();
    let mut index = 0;
    let mut tmp_token: Vec<char> = vec![];
    while index < token_char_vec.len() {
        let char = token_char_vec.get(index).unwrap();
        match char {
            '[' | ']' | '(' | ')' | '{' | '}' | '\u{300C}' | '\u{300D}' | '\u{300E}'
            | '\u{300F}' | '\u{3011}' | '\u{3010}' | '\u{FF08}' | '\u{FF09}' => {
                if !tmp_token.is_empty() {
                    let token = tmp_token.iter().cloned().collect::<String>();
                    all_token.push(token);
                }
                tmp_token = vec![];
            }
            _ => {
                tmp_token.push(char.to_owned());
            }
        }
        index += 1;
    }
    if !tmp_token.is_empty() {
        let token = tmp_token.iter().cloned().collect::<String>();
        all_token.push(token);
    }
    all_token
}