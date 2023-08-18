use crate::{predictiontree::PredictionTree, string, files, string::format};

pub fn load_tree_from_files(files: Vec<String>) -> PredictionTree {
    let mut tree = PredictionTree::new_empty();

    for file_name in files {
        load_tree_from_sentences(files::get_sentences(&file_name), &mut tree);
    }

    tree
}

fn load_tree_from_sentences(sentences: Vec<String>, tree: &mut PredictionTree) {
    for s in sentences {
        load_from_sentence(s, tree);
    }
}

fn load_from_sentence(sentence: String, tree: &mut PredictionTree) {
    let words = string::split_sentence(&sentence);
    if words.len()>2 {
        for i in 0..words.len()-2 {
            tree.insert(
                format(words.get(i).unwrap().to_string()), 
                format(words.get(i+1).unwrap().to_string()), 
                format(words.get(i+2).unwrap().to_string())
            );
        }
    }
}
