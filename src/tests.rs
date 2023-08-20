use crate::string;

#[test]
fn test_remove_last_word() {
    let str = String::from("Hello my name is adrien and i love pizza");
    let without_last = string::remove_last_word(&str);
    assert_eq!(without_last, "Hello my name is adrien and i love");
}

#[test]
fn test_last_words() {
    let example = String::from("Hello my name is adrien and i love pizza");
    let results = string::last_words(&example, 9).unwrap();
    let splited: Vec<&str> = example.split_whitespace().collect();
    assert_eq!(results, splited);

    let results = string::last_words(&example, 2).unwrap();
    assert_eq!(results, ["love", "pizza"]);
}
