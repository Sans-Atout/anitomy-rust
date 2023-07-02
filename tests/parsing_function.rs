use anitomy_rust::parsing::{is_crc32, is_resolution, ordinals_to_nb};

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
