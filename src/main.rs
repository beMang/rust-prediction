use std::io::stdin;

use crate::predictiontree::nice_print_of_possibilities;

pub mod files;
pub mod string;
pub mod tree;
pub mod singlethreadparser;
pub mod predictiontree;

fn get_files_to_parse() -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    
    for i in 1..209 {
        files.push(format!("./tweets/part_{}.txt", i));
    }

    files
}

fn main() {
    let mut exit = false;

    println!("Loading...");
    let tree = singlethreadparser::load_tree_from_files(get_files_to_parse());
    println!("Finished !");

    while !exit {
        let mut word1 = String::new();
        let mut word2 = String::new();

        stdin().read_line(&mut word1).expect("Failed to read user input");

        if word1.as_str().trim() == "exit" {
            exit=true;
            continue;
        }

        stdin().read_line(&mut word2).expect("Failed to read user input");

        word1 = string::format(word1);
        word2 = string::format(word2);

        match tree.retrieve_possibilities(&word1, &word2) {
            None => println!("No result for those two words"),
            Some(possibilities) => nice_print_of_possibilities(possibilities)
        }

        println!("Voulez-vous faire une autre prédiction")
    }
}

//_____TEST__________
fn _test() {
    //TEST TREES

    println!("Welcome to twitoz but in rust !");

    let mut my_tree = tree::Tree::new("Adrien", 20);
    my_tree.insert("Victor", 18);
    my_tree.insert("Adolph", 55);

    println!("{}", my_tree.get(&"bite").unwrap_or(&0));

    let mut binding = 55; 
    let myref = my_tree.get_mut(&"lul").unwrap_or(&mut binding);
    println!("Age : {}", myref);

    my_tree.print();

    //TEST STRING

    let example = String::from("Hello my name is adrien and i love pizza");
    let example2 = String::from("hello");
    let results = string::last_words(&example, 9).unwrap();
    let without_last = string::remove_last_word(&example2);

    for i in results {
        println!("{}", i);
    }

    println!("{}", without_last);


    // TEST FILES
    let file_test = false;

    if file_test {
        let files = get_files_to_parse();

        for file_name in files {
            let sentences = files::get_sentences(file_name.as_str());
            for i in sentences {
                println!("{}", i);
            }
        }
    }
}