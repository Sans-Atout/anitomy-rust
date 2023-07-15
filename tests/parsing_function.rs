use anitomy_rust::{
    elements::{Elements,Category},
    parsing::{contains_digit, is_anime_year, is_crc32, is_resolution, ordinals_to_nb},
    tokenizer::SubToken,
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
fn anime_year(){
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
fn match_single_ep_patern() {
    let sub_token = SubToken::new("01v2");
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::ReleaseVersion, "2");
    assert!(sub_token.match_episode_patern(&mut tmp_e));
    assert_eq!(tmp_e,wanted_element);
}

#[test]
fn match_multiple_ep_patern_01() {
    let sub_token = SubToken::new("01v2-03");
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumberAlt, "03")
        .add(Category::ReleaseVersion, "2");
    assert!(sub_token.match_episode_patern(&mut tmp_e));
    assert_eq!(tmp_e,wanted_element);
}

#[test]
fn match_multiple_ep_patern_02() {
    let sub_token = SubToken::new("01-03v2");
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumberAlt, "03")
        .add(Category::ReleaseVersion, "2");
    assert!(sub_token.match_episode_patern(&mut tmp_e));
    assert_eq!(tmp_e,wanted_element);
}

#[test]
fn match_multiple_ep_patern_03() {
    let sub_token = SubToken::new("01v1-03v2");
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumberAlt, "03")
        .add(Category::ReleaseVersion, "1")
        .add(Category::ReleaseVersion, "2");
    assert!(sub_token.match_episode_patern(&mut tmp_e));
    assert_eq!(tmp_e,wanted_element);
}

#[test]
fn match_multiple_ep_patern_04() {
    let sub_token = SubToken::new("01-03");
    let mut tmp_e = Elements::new();
    let wanted_element = Elements::new()
        .add(Category::EpisodeNumber, "01")
        .add(Category::EpisodeNumberAlt, "03");
    assert!(sub_token.match_episode_patern(&mut tmp_e));
    assert_eq!(tmp_e,wanted_element);
}