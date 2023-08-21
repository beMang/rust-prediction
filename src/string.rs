pub fn last_words(str: &String, n: u32) -> Option<Vec<&str>> {
    let mut words: Vec<&str> = str.rsplitn((n + 1) as usize, " ").collect();
    if words.len() < n as usize {
        return None;
    };
    if words.len() != n as usize {
        words.pop();
    }
    words.reverse();
    return Some(words);
}

pub fn remove_last_word(str: &String) -> &str {
    let words = str.rsplit_once(" ");
    match words {
        Some((t1, _t2)) => return t1,
        None => return "",
    }
}

pub fn split_sentence(str: &String) -> Vec<&str> {
    str.split_whitespace().collect()
}

pub fn format(str: String) -> String {
    String::from(str.to_lowercase().trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_last_word() {
        let str = String::from("Hello my name is adrien and i love pizza");
        let without_last = remove_last_word(&str);
        assert_eq!(without_last, "Hello my name is adrien and i love");
    }

    #[test]
    fn test_last_words() {
        let example = String::from("Hello my name is adrien and i love pizza");
        let results = last_words(&example, 9).unwrap();
        let splited: Vec<&str> = example.split_whitespace().collect();
        assert_eq!(results, splited);

        let results = last_words(&example, 2).unwrap();
        assert_eq!(results, ["love", "pizza"]);
    }
}
