use anitomy_rust::split::split_by_delimiter;

#[test]
fn split_regex() {
    let d: Vec<char> = vec![' ', '_', '.', '&', '+', ',', '|'];
    let s = "hello_world I.m&a+beautifull,rust|test";
    let e: Vec<&str> = vec![
        "hello",
        "world",
        "I",
        "m",
        "a",
        "beautifull",
        "rust",
        "test",
    ];
    assert_eq!(e, split_by_delimiter(s, d))
}