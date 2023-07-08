use anitomy_rust::{tokenizer::{tokenize, Token, SubToken, SubTokenCategory}, elements::Elements};
use anitomy_rust::elements::Category;

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

#[test]
fn token_parsing(){
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let mut tmp_e = Elements::new();
    let mut t = Token::new("40F2A957", &d, true);
    tmp_e = t.parse(&mut tmp_e);
    assert!(!tmp_e.is_category_empty(Category::FileChecksum))
}

#[test]
fn found_a_token(){
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let mut tmp_e = Elements::new();
    let mut t = Token::new("40F2A957", &d, true);
    tmp_e = t.keyword_found(Category::FileChecksum, 0, &mut tmp_e);
    assert_eq!(tmp_e,Elements::new().add(Category::FileChecksum, "40F2A957"));
}

#[test]
fn contain_unknow(){
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let mut tmp_e = Elements::new();
    let mut t = Token::new("40F2A957", &d, true);
    assert!(t.contains_unknow());
    let _e = t.parse(&mut tmp_e);
    assert!(!t.contains_unknow())
}

#[test]
fn is_subtoken_found(){
    assert!(!SubToken::new("40F2A957").is_found());
    assert!(SubToken::new("40F2A957").category(SubTokenCategory::Found).is_found());
}