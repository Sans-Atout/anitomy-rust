use anitomy_rust::{elements::Category, keyword::{Keyword, Manager}};

#[test]
fn keyword_001() {
    let keyword = Keyword::new(Category::AnimeSeason)
        .identifiable(false)
        .valid(false)
        .searchable(false);
    assert!(!keyword.is_identifiable());
    assert!(!keyword.is_searchable());
    assert!(!keyword.is_valid());
}

#[test]
fn keyword_002() {
    let keyword = Keyword::new(Category::AnimeSeason)
        .identifiable(false)
        .searchable(false);
    assert!(!keyword.is_identifiable());
    assert!(!keyword.is_searchable());
    assert!(keyword.is_valid());
}

#[test]
fn keyword_003() {
    let keyword = Keyword::new(Category::AnimeSeason)
        .identifiable(false)
        .valid(false);
    assert!(!keyword.is_identifiable());
    assert!(keyword.is_searchable());
    assert!(!keyword.is_valid());
}

#[test]
fn keyword_004() {
    let keyword = Keyword::new(Category::AnimeSeason)
        .searchable(false)
        .valid(false);
    assert!(keyword.is_identifiable());
    assert!(!keyword.is_searchable());
    assert!(!keyword.is_valid());
}

#[test]
fn keyword_005() {
    let keyword = Keyword::new(Category::AnimeSeason)
        .valid(false);
    assert!(keyword.is_identifiable());
    assert!(keyword.is_searchable());
    assert!(!keyword.is_valid());
}

#[test]
fn keyword_006() {
    let keyword = Keyword::new(Category::AnimeSeason)
        .searchable(false);
    assert!(keyword.is_identifiable());
    assert!(!keyword.is_searchable());
    assert!(keyword.is_valid());
}

#[test]
fn keyword_007() {
    let keyword = Keyword::new(Category::AnimeSeason)
        .identifiable(false);
    assert!(!keyword.is_identifiable());
    assert!(keyword.is_searchable());
    assert!(keyword.is_valid());
}

#[test]
fn keyword_008() {
    let keyword = Keyword::new(Category::AnimeSeason);
    assert!(keyword.is_identifiable());
    assert!(keyword.is_searchable());
    assert!(keyword.is_valid());
}

#[test]
fn manager_find(){
    let manager = Manager::new();
    assert!(manager.find("VOLUME").is_some());
    assert!(manager.find("My CUSTOM KEYWORD").is_none());
}