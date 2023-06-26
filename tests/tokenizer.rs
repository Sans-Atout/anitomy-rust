use anitomy_rust::tokenizer::tokenize;

#[test]
fn non_normal_situation() {
    let tokens = tokenize("[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4");
    let wanted_token: Vec<String> = vec![
        "Taka".to_string(),
        "_Fullmetal_Alchemist_".to_string(),
        "2009".to_string(),
        "_04_".to_string(),
        "720p".to_string(),
        "40F2A957".to_string(),
        ".mp4".to_string()
    ];
    assert_eq!(tokens,wanted_token)
}

#[test]
fn normal_situation() {
    let tokens = tokenize("[Taka]{Fullmetal_Alchemist}(2009)_04_[720p][40F2A957]");
    let wanted_token: Vec<String> = vec![
        "Taka".to_string(),
        "Fullmetal_Alchemist".to_string(),
        "2009".to_string(),
        "_04_".to_string(),
        "720p".to_string(),
        "40F2A957".to_string(),
    ];
    assert_eq!(tokens,wanted_token)
}