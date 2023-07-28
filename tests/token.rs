use anitomy_rust::elements::Category;
use anitomy_rust::token::subtoken::{SubToken, SubTokenCategory};
use anitomy_rust::token::main_token::Token;
use anitomy_rust::elements::Elements;

#[test]
fn token_parsing() {
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let mut _tmp_e = Elements::new();
    let mut _t = Token::new("40F2A957", &d, true,true);
    // TODO insert parsing a single token
    assert!(!_tmp_e.is_category_empty(Category::FileChecksum))
}

#[test]
fn contain_unknow() {
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let _tmp_e = Elements::new();
    let t = Token::new("40F2A957", &d, true,true);
    assert!(t.contains_unknow());
    // TODO insert parsing a single token
}

#[test]
fn is_subtoken_found() {
    assert!(!SubToken::new("40F2A957").is_category(SubTokenCategory::Found));
    assert!(SubToken::new("40F2A957")
        .category(SubTokenCategory::Found)
        .is_category(SubTokenCategory::Found));
}
