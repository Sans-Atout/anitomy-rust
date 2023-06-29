use anitomy_rust::tokenizer::{tokenize, Token};

#[test]
fn non_normal_situation() {
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let tokens = tokenize(
        "[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4",
        &d,
    );
    let wanted_token: Vec<Token> = vec![
        Token::new("Taka", &d, true),
        Token::new("_Fullmetal_Alchemist_", &d, false),
        Token::new("2009", &d, true),
        Token::new("_04_", &d, false),
        Token::new("720p", &d, true),
        Token::new("40F2A957", &d, true),
        Token::new(".mp4", &d, false),
    ];
    assert_eq!(tokens, wanted_token)
}

#[test]
fn normal_situation() {
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let tokens = tokenize("[Taka]Fullmetal_Alchemist(2009)_04_{720p}[40F2A957]", &d);
    let wanted_token: Vec<Token> = vec![
        Token::new("Taka", &d, true),
        Token::new("Fullmetal_Alchemist", &d, false),
        Token::new("2009", &d, true),
        Token::new("_04_", &d, false),
        Token::new("720p", &d, true),
        Token::new("40F2A957", &d, true),
    ];
    assert_eq!(tokens, wanted_token)
}
