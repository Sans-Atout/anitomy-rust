use anitomy_rust::{elements::{Elements, Category}, tokenizer::SubToken};

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