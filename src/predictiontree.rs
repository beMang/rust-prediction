use std::collections::HashMap;

use crate::tree::Tree;

pub struct PredictionTree {
    pub tree: Tree<String, Tree<String, HashMap<String, u32>>>
}

impl PredictionTree {
    pub fn new_empty() -> PredictionTree{
        return PredictionTree { tree: Tree::new_empty() }
    }

    pub fn insert(&mut self, word1: String, word2: String, word3: String) {
        match self.tree.get_mut(&word1) {
            None => {
                let mut possibility = HashMap::new();
                possibility.insert(word3, 1 as u32);

                self.tree.insert(word1, Tree::new(word2, possibility));
            },
            Some(second_tree) => {
                match second_tree.get_mut(&word2) {
                    None => {
                        let mut possibility = HashMap::new();
                        possibility.insert(word3, 1 as u32);

                        second_tree.insert(word2, possibility);
                    },
                    Some(possibility) => {
                        let count = possibility.entry(word3).or_insert(0);
                        *count+=1;
                    }
                }
            }
        }
    }

    pub fn retrieve_possibilities(&self, word1: &String, word2: &String) -> Option<&HashMap<String, u32>> {
        return match self.tree.get(word1) {
            None => None,
            Some(second_tree) => {
                second_tree.get(word2)
            }
        }
    }
}

pub fn nice_print_of_possibilities(possibilities: &HashMap<String, u32>) {
    let total = possibilities.iter().fold(0, |acc, x| acc + x.1 ) as f32;

    let mut sorted = Vec::from_iter(possibilities.iter());
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    println!("Possibilités pour cette combinaison :");
    for p in sorted {
        println!("{} - {}", p.0, *p.1 as f32/total);
    }
}