use std::fmt;

use egui::{Color32, RichText, Ui};

use crate::{predictiontree::{PredictionTree, nice_print_of_possibilities}, string::last_words, singlethreadparser};

/**
 * Application state
 */
#[derive(PartialEq)]
pub enum State {
    Loading,
    Ready,
    Exit,
}

impl State {
    fn new() -> State {
        State::Loading
    }

    pub fn ready(&mut self) {
        *self = State::Ready;
    }

    pub fn loading(&mut self) {
        *self = State::Loading;
    }

    pub fn exit(&mut self) {
        *self = State::Exit
    }
}

/**
 * Display trait, qui permet d'avoir la methode to_string
 */
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            State::Loading => "Loading...",
            State::Exit => "Exiting...",
            State::Ready => "Ready !"
        };
        write!(f, "{}", value)
    }
}

pub struct App {
    prediction_tree: PredictionTree,
    state: State,
    ouput: String,
    input: String,
}

impl App {
    pub fn new() -> App {
        return App {
            prediction_tree: PredictionTree::new_empty(),
            state: State::new(),
            input: String::new(),
            ouput: String::new(),
        };
    }

    /**
     * Fonction chargée de charger la base de donnée
     */
    fn load_sets(&mut self) {
        self.state.loading();
        self.prediction_tree = singlethreadparser::load_tree_from_files(Self::get_files_to_parse("./french/"));
        singlethreadparser::add_files_to_tree(Self::get_files_to_parse("./tweets/"), &mut self.prediction_tree); //we can add some more data
        self.state.ready();
    }

    fn get_files_to_parse(dir: &str) -> Vec<String> {
        let mut files = crate::files::files_in_dir(dir).expect("Failed to read dir");
    
        for f in &mut files {
            f.insert_str(0, dir);
        }
        return files;
    }

    fn build_central_panel(&mut self, ui: &mut Ui) {
        match self.input.chars().last() {
            Some(last) => {
                if last == ' ' {
                    //we could try to make a prediction :)
                }
            },
            _ => {}
        }
        ui.text_edit_multiline(&mut self.input);

        ui.label(&self.ouput);
        if ui
            .button(
                RichText::new("Prédire")
                    .background_color(Color32::from_rgb(0, 50, 255))
                    .size(20.2),
            )
            .clicked()
        {
            let last_words = last_words(& self.input, 2).unwrap(); //On récupère les 2 derniers mots
            match self.prediction_tree.retrieve_possibilities(last_words.get(0).unwrap(), last_words.get(1).unwrap()) {
                None => {
                    self.ouput = "No result for thoses 2 words".to_string();
                },
                Some(possibilities) => {
                    self.ouput = nice_print_of_possibilities(possibilities)
                },
            }
        }
    }

    fn build_top_pannel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("rust-prediction, a better tool than twitoz !")
                    .size(20.2)
                    .color(Color32::from_rgb(255, 50, 50)),
            );
            ui.add_space(20.0);
            ui.label(format!("State : {}", self.state.to_string()));
        });
    }

}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("pannel").show(ctx, |ui| {
            self.build_top_pannel(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                self.build_central_panel(ui);
            })
        });

        if self.state == State::Loading {
            self.load_sets();
        }
    }
 }