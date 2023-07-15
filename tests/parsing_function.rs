use anitomy_rust::parsing::number::{is_crc32, is_resolution, ordinals_to_nb, is_anime_year, contains_digit, is_hexa, is_digit};

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