pub fn last_words(str:& String, n: u32) -> Option<Vec<&str>> {
    let mut words: Vec<&str> = str.rsplitn((n+1) as usize, " ").collect();
    if words.len()<n as usize {return None};
    if words.len()!=n as usize {words.pop();}
    words.reverse();
    return Some(words);
}

pub fn remove_last_word(str: &String) -> &str {
    let words = str.rsplit_once(" ");
    match words {
        Some((t1, _t2)) => return t1,
        None => return ""
    }
}

pub fn split_sentence(str: &String) -> Vec<&str> {
    str.split_whitespace().collect()
}

pub fn format(str: String) -> String {
    String::from(str.to_lowercase().trim())
}