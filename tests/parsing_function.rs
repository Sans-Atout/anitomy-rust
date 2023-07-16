use anitomy_rust::{
    elements::{Category, Elements, Element},
    parsing::{
        episode::{
            match_fractal_episode, match_japanese_counter, match_multiple_ep,
            match_partial_episode_pattern, match_season_ep_patern, match_type_episode,
            parse_single_ep, parse_single_subtoken,
        },
        number::{
            contains_digit, is_anime_year, is_crc32, is_digit, is_hexa, is_resolution,
            ordinals_to_nb,
        }, string::{parse_anime_title, parse_release_group, parse_episode_title},
    },
    tokenizer::Token,
};

#[test]
fn crc32() {
    assert!(is_crc32("C09462E2"));
    assert!(is_crc32("8F59F2BA"));
    assert!(!is_crc32("8F5GF2BA"));
    assert!(!is_crc32("8F50F2BAA"));
    assert!(!is_crc32("8F50F2B"));
}

#[test]
fn resolution() {
    assert!(is_resolution("1080p"));
    assert!(is_resolution("2K"));
    assert!(is_resolution("1920x1080"));
    assert!(is_resolution("720p"));
    assert!(!is_resolution("Hello world"));
    assert!(is_resolution("1080×720"));
}

#[test]
fn ordinal() {
    assert_eq!(ordinals_to_nb("First"), "1");
    assert_eq!(ordinals_to_nb("second"), "2");
    assert_eq!(ordinals_to_nb("5th"), "5");
    assert_eq!(ordinals_to_nb("5the"), "");
}

#[test]
fn anime_year() {
    assert!(is_anime_year("2009"));
    assert!(is_anime_year("1920"));
    assert!(!is_anime_year("1400"));
    assert!(is_anime_year("2050"));
    assert!(!is_anime_year("2500"));
    assert!(!is_anime_year("gdyuighjdhgj"));
}

#[test]
fn test_contains_number() {
    assert!(contains_digit("E01S02"));
    assert!(contains_digit("032"));
    assert!(!contains_digit("A"));
    assert!(!contains_digit("Hello World"));
}

#[test]
fn test_hexa() {
    assert!(is_hexa("028934"));
    assert!(is_hexa("FFF"));
    assert!(is_hexa("0123456789ABCDEF"));
    assert!(!is_hexa("G015021"));
    assert!(is_hexa("acbdef"));
    assert!(!is_hexa("00000fg"))
}

#[test]
fn test_isdigit() {
    assert!(is_digit("FFF"));
    assert!(is_digit("256"));
    assert!(is_digit("-120"));
    assert!(!is_digit("Hello World"));
}

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
    let d: Vec<char> = vec![' ', '_', '.','-', '&', '+', ',', '|'];
    let mut e = Elements::new();
    let mut parsing_data = vec![
        Token::new("Kira-Fansub", &d, true),
        Token::new(" Uchuu no Stellvia ep 14 ", &d, false),
        Token::new("BD 1280x960 24fps AAC", &d, true),
        Token::new("06EE7355", &d, true),
    ];
    parse_release_group(&mut parsing_data, &mut e);
    let tested = e.find(Category::ReleaseGroup).unwrap();
    let wanted = Element::new(Category::ReleaseGroup, "Kira Fansub");
    assert_eq!( tested,wanted )
}

#[test]
fn test_find_anime_title_001(){
    let d: Vec<char> = vec![' ', '_', '.','-', '&', '+', ',', '|'];
    let mut e = Elements::new();
    let mut anime_title_subtoken: Token = Token::new(" Uchuu no Stellvia ep 14 ", &d, false);
    let subtoken = anime_title_subtoken.sub_tokens();
    subtoken[4].category(anitomy_rust::tokenizer::SubTokenCategory::Found);
    subtoken[5].category(anitomy_rust::tokenizer::SubTokenCategory::Found);
    let mut parsing_data = vec![
        Token::new("Kira-Fansub", &d, true),
        anime_title_subtoken,
        Token::new("BD 1280x960 24fps AAC", &d, true),
        Token::new("06EE7355", &d, true),
    ];

    parse_anime_title(&mut parsing_data,&mut e);
    let tested = e.find(Category::AnimeTitle).unwrap();
    let wanted = Element::new(Category::AnimeTitle, "Uchuu no Stellvia");
    assert_eq!( tested,wanted )
}

#[test]
fn test_find_anime_title_002(){
    let d: Vec<char> = vec![' ', '_', '.','-', '&', '+', ',', '|'];
    let mut e = Elements::new();
    let mut anime_title_subtoken: Token = Token::new(" Uchuu no Stellvia ep 14 ", &d, true);
    let subtoken = anime_title_subtoken.sub_tokens();
    subtoken[4].category(anitomy_rust::tokenizer::SubTokenCategory::Found);
    subtoken[5].category(anitomy_rust::tokenizer::SubTokenCategory::Found);
    let mut parsing_data = vec![
        Token::new("Kira-Fansub", &d, true),
        anime_title_subtoken,
        Token::new("BD 1280x960 24fps AAC", &d, true),
        Token::new("06EE7355", &d, true),
    ];

    parse_anime_title(&mut parsing_data,&mut e);
    let tested = e.find(Category::AnimeTitle).unwrap();
    let wanted = Element::new(Category::AnimeTitle, "Uchuu no Stellvia");
    assert_eq!( tested,wanted )
}

#[test]
fn test_find_episode_title(){
    let d: Vec<char> = vec![' ', '_', '.','-', '&', '+', ',', '|'];
    let mut e = Elements::new();
    let mut anime_title_subtoken: Token = Token::new(" Uchuu no Stellvia ep 14 My Super Episode title", &d, false);
    let subtoken = anime_title_subtoken.sub_tokens();
    subtoken[4].category(anitomy_rust::tokenizer::SubTokenCategory::Found);
    subtoken[5].category(anitomy_rust::tokenizer::SubTokenCategory::Found);
    let mut parsing_data = vec![
        Token::new("Kira-Fansub", &d, true),
        anime_title_subtoken,
        Token::new("BD 1280x960 24fps AAC", &d, true),
        Token::new("06EE7355", &d, true),
    ];

    parse_episode_title(&mut parsing_data,&mut e);
    let tested = e.find(Category::EpisodeTitle).unwrap();
    let wanted = Element::new(Category::EpisodeTitle, "Uchuu no Stellvia");
    assert_eq!( tested,wanted )
}