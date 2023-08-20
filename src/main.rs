use std::io::stdin;

use egui::{RichText, Color32};

use crate::predictiontree::nice_print_of_possibilities;

pub mod files;
pub mod string;
pub mod singlethreadparser;
pub mod predictiontree;

#[cfg(test)]
mod tests;

fn get_files_to_parse() -> Vec<String> {
    let dir = "./tweets/";
    let mut files = files::files_in_dir(dir).expect("Failed to read dir");

    for f in &mut files {
        f.insert_str(0, dir);
    }
    return files;
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

    let mut counter = 0;
    
    let options = eframe::NativeOptions::default();
    let end_state = eframe::run_simple_native("A little counter", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("Hey this is a title").size(20.2).color(Color32::from_rgb(255, 0, 0)));
            ui.horizontal(|ui| {
                if ui.button("-").clicked() {
                    counter -= 1;
                }
                ui.label(counter.to_string());
                if ui.button("+").clicked() {
                    counter += 1;
                }
            });
        });
    });

    match end_state {
        Err(e) => {
            println!("Erreur while running : {}", e.to_string())
        },
        _ => {}
    }
}
