use anitomy_rust::parsing::is_crc32;

#[test]
fn crc32(){
    assert!(is_crc32("C09462E2"));
    assert!(is_crc32("8F59F2BA"));
    assert!(!is_crc32("8F5GF2BA"));
    assert!(!is_crc32("8F50F2BAA"));
    assert!(!is_crc32("8F50F2B"));
}