pub mod files;
pub mod string;
pub mod tree;

fn main() {
    println!("Welcome to twitoz but in rust !");

    let mut my_tree = tree::Tree::new("Adrien", 20);
    my_tree.insert("Victor", 18);

    println!("{}", my_tree.get("bite").unwrap_or(&0));

    let example = String::from("Hello my name is adrien and i love pizza");
    let example2 = String::from("hello");
    let results = string::last_words(&example, 9).unwrap();
    let without_last = string::remove_last_word(&example2);

    for i in results {
        println!("{}", i);
    }

    println!("{}", without_last);
}
