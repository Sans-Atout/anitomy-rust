use anitomy_rust::{split::{split_token, split_raw_data}, token::token::Token};

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
    assert_eq!(e, split_token(s, &d))
}

#[test]
fn non_normal_split(){
    let d: &[char; 8] = &[' ', '_', '.', '-', '&', '+', ',', '|'];
    let tested = split_raw_data("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv",d);
    let mut wanted = vec![
        Token::new("TaigaSubs", d, true, false),
        Token::new("_Toradora!_", d, false, false),
        Token::new("2008", d, true, true),
        Token::new("_-_01v2_-_Tiger_and_Dragon_", d, false, false),
        Token::new("1280x720_H.264_FLAC", d, true, false),
        Token::new("1234ABCD", d, true, false),
    ];
    assert_ne!(wanted,tested);
    wanted.push(Token::new(".mkv", d, false, false));
    assert_eq!(wanted,tested);
}

#[test]
fn normal_split(){
    let d: &[char; 8] = &[' ', '_', '.', '-', '&', '+', ',', '|'];
    let tested = split_raw_data("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD]",d);
    let wanted = vec![
        Token::new("TaigaSubs", d, true, false),
        Token::new("_Toradora!_", d, false, false),
        Token::new("2008", d, true, true),
        Token::new("_-_01v2_-_Tiger_and_Dragon_", d, false, false),
        Token::new("1280x720_H.264_FLAC", d, true, false),
        Token::new("1234ABCD", d, true, false),
    ];
    assert_eq!(wanted,tested);
}

#[test]
fn test_split_sub_token(){
    let tested = split_token("_-_01v2_-_Tiger_and_Dragon_", &[' ', '_', '.', '-', '&', '+', ',', '|']);
    let wanted = vec![
        "01v2",
        "-",
        "Tiger",
        "and",
        "Dragon"
    ];

    assert_eq!(wanted,tested);
}