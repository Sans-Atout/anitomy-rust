use anitomy_rust::{
    elements::{Category, Element},
    Parser,
};

#[test]
fn testing_anime_season() {
    // TODO
}
#[test]
fn testing_anime_season_prefix() {
    // TODO
}
#[test]
fn testing_anime_title() {
    // TODO
}
#[test]
fn testing_anime_type() {
    // TODO
}
#[test]
fn testing_anime_year() {
    // TODO
}
#[test]
fn testing_audio_term() {
    // TODO
}
#[test]
fn testing_device_compatibility() {
    // TODO
}
#[test]
fn testing_episode_number() {
    // TODO
}
#[test]
fn testing_episode_number_alt() {
    // TODO
}
#[test]
fn testing_episode_prefix() {
    // TODO
}
#[test]
fn testing_episode_title() {
    // TODO
}
#[test]
fn testing_file_checksum() {
    let elements = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::FileChecksum);
    assert!(tested.is_ok());
    let wanted = Element::new(Category::FileChecksum, "1234ABCD");
    assert_eq!(tested.unwrap(),wanted)
}
#[test]
fn testing_file_extension() {
    let elements = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::FileExtension);
    assert!(tested.is_ok());
    let wanted = Element::new(Category::FileExtension, "mkv");
    assert_eq!(tested.unwrap(),wanted)
}
#[test]
fn testing_file_name() {
    // TODO
}
#[test]
fn testing_language() {
    // TODO
}
#[test]
fn testing_other() {
    // TODO
}
#[test]
fn testing_release_group() {
    // TODO
}
#[test]
fn testing_release_information() {
    // TODO
}
#[test]
fn testing_release_version() {
    // TODO
}
#[test]
fn testing_source() {
    // TODO
}
#[test]
fn testing_subtitles() {
    // TODO
}
#[test]
fn testing_video_resolution() {
    let elements = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").parse();
    assert!(elements.is_ok());
    let tested = elements.unwrap().find(Category::VideoResolution);
    assert!(tested.is_ok());
    let wanted = Element::new(Category::VideoResolution, "1280x720");
    assert_eq!(tested.unwrap(),wanted)
}
#[test]
fn testing_video_term() {
    // TODO
}
#[test]
fn testing_volume_number() {
    // TODO
}
#[test]
fn testing_volume_prefix() {
    // TODO
}
#[test]
fn testing_unknown() {
    // TODO
}
