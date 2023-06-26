use anitomy_rust::utils::remove_ignored_string;

#[test]
fn remove_one_string() {
    let tested_string = "Hello World!";
    let r1 = remove_ignored_string(tested_string, vec!["World".to_string()]);
    assert_eq!(r1, "Hello !");
}

#[test]
fn remove_multiple() {
    let tested_string = "Hello World!";
    let r2 = remove_ignored_string(
        tested_string,
        vec!["World".to_string(), "Hello".to_string()],
    );
    assert_eq!(r2, " !");
}

#[test]
fn nothing_to_remove() {
    let tested_string = "EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv";
    let r2 = remove_ignored_string(tested_string, vec!["['EvoBot.']".to_string()]);
    assert_eq!(r2, tested_string);
}

#[test]
fn real_test_remove() {
    let tested_string = "EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv";
    let r2 = remove_ignored_string(tested_string, vec!["EvoBot.".to_string()]);
    assert_eq!(
        r2,
        "[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv"
    );
}
