/*
use anitomy_rust::{
    elements::{Category, Element, Elements},
    parsing::{
        episode::{
            match_fractal_episode, match_japanese_counter, match_multiple_ep,
            match_partial_episode_pattern, match_season_ep_patern, match_type_episode,
            parse_single_ep, parse_single_subtoken,
        },
        string::{parse_anime_title, parse_episode_title, parse_release_group},
    },
};



#[test]
fn match_single_ep_patern() {
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::ReleaseVersion, "2");
    assert!(parse_single_ep("01v2", &mut tmp_e));
    assert_eq!(tmp_e, wanted_element);
}

#[test]
fn match_multiple_ep_patern_01() {
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumberAlt, "03")
        .add(Category::ReleaseVersion, "2");
    assert!(match_multiple_ep("01v2-03", &mut tmp_e));
    assert_eq!(tmp_e, wanted_element);
}

#[test]
fn match_multiple_ep_patern_02() {
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumberAlt, "03")
        .add(Category::ReleaseVersion, "2");
    assert!(match_multiple_ep("01-03v2", &mut tmp_e));
    assert_eq!(tmp_e, wanted_element);
}

#[test]
fn match_multiple_ep_patern_03() {
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumberAlt, "03")
        .add(Category::ReleaseVersion, "1")
        .add(Category::ReleaseVersion, "2");
    assert!(match_multiple_ep("01v1-03v2", &mut tmp_e));
    assert_eq!(tmp_e, wanted_element);
}

#[test]
fn match_multiple_ep_patern_04() {
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumberAlt, "03");
    assert!(match_multiple_ep("01-03", &mut tmp_e));
    assert_eq!(tmp_e, wanted_element);
}

#[test]
fn match_episode_type() {
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::AnimeType, "ONA")
        .add(Category::EpisodeNumber, "01")
        .add(Category::ReleaseVersion, "3");
    assert!(match_type_episode("ONA01v3", &mut tmp_e, &d));
    assert_eq!(tmp_e, wanted_element);
    tmp_e = Elements::new();
    assert!(!match_type_episode("ONAFail", &mut tmp_e, &d));
    assert_eq!(tmp_e, Elements::new());
}

#[test]
fn test_japanese_ep() {
    let mut tmp_e = Elements::new();
    let wanted = Elements::new().add(Category::EpisodeNumber, "125");
    assert!(!match_japanese_counter("ONAFail", &mut tmp_e));
    assert!(match_japanese_counter("125話", &mut tmp_e));
    assert_eq!(tmp_e, wanted);
}

#[test]
fn test_fractal() {
    let mut tmp_e = Elements::new();
    let wanted = Elements::new().add(Category::EpisodeNumber, "1.5");
    assert!(!match_fractal_episode("11.1", &mut tmp_e));
    assert!(match_fractal_episode("1.5", &mut tmp_e));
    assert_eq!(tmp_e, wanted);
}

#[test]
fn test_parse_single_subtoken() {
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let mut e = Elements::new();
    assert!(parse_single_subtoken(&d, "01v2", &mut e));
    assert!(parse_single_subtoken(&d, "1.5", &mut e));
    assert!(match_fractal_episode("1.5", &mut e));
    assert!(parse_single_subtoken(&d, "01-03", &mut e));
    assert!(parse_single_subtoken(&d, "S01E02", &mut e));
    assert!(parse_single_subtoken(&d, "ONA1.5", &mut e));
    assert!(parse_single_subtoken(&d, "125話", &mut e));
    assert!(!parse_single_subtoken(&d, "03-02", &mut e));
    assert!(parse_single_subtoken(&d, "125A", &mut e));
    assert!(parse_single_subtoken(&d, "125a", &mut e));
    assert!(parse_single_subtoken(&d, "#32v1", &mut e));
}

#[test]
fn test_season_ep_patern() {
    let mut e = Elements::new();
    assert!(match_season_ep_patern("S02E01", &mut e));
    assert!(match_season_ep_patern("S02E01-03", &mut e));
    assert!(!match_season_ep_patern("SAE01", &mut e));
    assert!(match_season_ep_patern("S01-02E01", &mut e));
    assert!(match_season_ep_patern("01x02", &mut e));
}

#[test]
fn test_match_partial_episode_pattern() {
    let mut e = Elements::new();
    assert!(match_partial_episode_pattern("125a", &mut e));
}

#[test]
fn test_find_release_group() {
    let d: Vec<char> = vec![' ', '_', '.', '-', '&', '+', ',', '|'];
    let mut e = Elements::new();
    let mut parsing_data = vec![
        Token::new("Kira-Fansub", &d, true, false),
        Token::new(" Uchuu no Stellvia ep 14 ", &d, false, false),
        Token::new("BD 1280x960 24fps AAC", &d, true, false),
        Token::new("06EE7355", &d, true, true),
    ];
    parse_release_group(&mut parsing_data, &mut e, &d);
    let tested = e.find(Category::ReleaseGroup).unwrap();
    let wanted = Element::new(Category::ReleaseGroup, "Kira-Fansub");
    assert_eq!(tested, wanted)
}

#[test]
fn test_find_anime_title_001() {
    let d: Vec<char> = vec![' ', '_', '.', '-', '&', '+', ',', '|'];
    let mut e = Elements::new();
    let mut anime_title_subtoken: Token = Token::new(" Uchuu no Stellvia ep 14 ", &d, false, false);
    let subtoken = anime_title_subtoken.sub_tokens();
    subtoken[3].category(SubTokenCategory::Found);
    subtoken[4].category(SubTokenCategory::Found);
    let mut parsing_data = vec![
        Token::new("Kira-Fansub", &d, true, false),
        anime_title_subtoken,
        Token::new("BD 1280x960 24fps AAC", &d, true, true),
        Token::new("06EE7355", &d, true, false),
    ];

    parse_anime_title(&mut parsing_data, &mut e, &d);
    let tested = e.find(Category::AnimeTitle).unwrap();
    let wanted = Element::new(Category::AnimeTitle, "Uchuu no Stellvia");
    assert_eq!(tested, wanted)
}

#[test]
fn test_find_anime_title_002() {
    let d: Vec<char> = vec![' ', '_', '.', '-', '&', '+', ',', '|'];
    let mut e = Elements::new();
    let mut anime_title_subtoken: Token = Token::new(" Uchuu no Stellvia ep 14 ", &d, true, false);
    let subtoken = anime_title_subtoken.sub_tokens();
    subtoken[3].category(SubTokenCategory::Found);
    subtoken[4].category(SubTokenCategory::Found);
    let mut parsing_data = vec![
        Token::new("Kira-Fansub", &d, true, false),
        anime_title_subtoken,
        Token::new("BD 1280x960 24fps AAC", &d, true, true),
        Token::new("06EE7355", &d, true, false),
    ];

    parse_anime_title(&mut parsing_data, &mut e, &d);
    let tested = e.find(Category::AnimeTitle).unwrap();
    let wanted = Element::new(Category::AnimeTitle, "Uchuu no Stellvia");
    assert_eq!(tested, wanted)
}

#[test]
fn test_find_episode_title() {
    let d: Vec<char> = vec![' ', '_', '.', '-', '&', '+', ',', '|'];
    let mut e = Elements::new();
    let mut anime_title_subtoken: Token = Token::new(
        " Uchuu no Stellvia ep 14 My Super Episode title",
        &d,
        false,
        false,
    );
    let subtoken = anime_title_subtoken.sub_tokens();
    subtoken[3].category(SubTokenCategory::Found);
    subtoken[4].category(SubTokenCategory::Found);
    let mut parsing_data = vec![
        Token::new("Kira-Fansub", &d, true, false),
        anime_title_subtoken,
        Token::new("BD 1280x960 24fps AAC", &d, true, true),
        Token::new("06EE7355", &d, true, false),
    ];

    parse_episode_title(&mut parsing_data, &mut e, &d);
    let tested = e.find(Category::EpisodeTitle).unwrap();
    let wanted = Element::new(Category::EpisodeTitle, "Uchuu no Stellvia");
    assert_eq!(tested, wanted)
}
*/