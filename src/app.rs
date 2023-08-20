use eframe::Frame;
use egui::{Color32, RichText, Ui, Vec2};

use crate::{predictiontree::{PredictionTree, nice_print_of_possibilities}, string::last_words, singlethreadparser};

#[derive(PartialEq)]
pub enum State {
    Loading(String),
    Ready(String),
    Exit,
}

impl State {
    fn new() -> State {
        State::Loading(String::from("Loading..."))
    }

    pub fn ready(&mut self) {
        *self = State::Ready(String::from("Ready !"));
    }

    pub fn loading(&mut self) {
        *self = State::Loading(String::from("Loading..."));
    }

    pub fn exit(&mut self) {
        *self = State::Exit
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

    pub fn run(mut self) -> Result<(), eframe::Error> {
        self.load_sets();
        let mut options = eframe::NativeOptions::default();
        options.resizable = true;
        options.drag_and_drop_support = true;
        options.decorated = true;
        

        let result = eframe::run_simple_native("rust-prediction", options, move |ctx, frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_max_size(Vec2 { x: 300.0, y: 600.0 });
                self.build_ui(ui, frame);
            });
        });
        result
    }

    /**
     * Fonction chargée de charger la base de donnée
     */
    fn load_sets(&mut self) {
        self.state.loading();
        self.prediction_tree = singlethreadparser::load_tree_from_files(Self::get_files_to_parse());
        self.state.ready();
    }

    fn build_ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        ui.label(
            RichText::new("rust-prediction, a better tool than twitoz !")
                .size(20.2)
                .color(Color32::from_rgb(255, 50, 50)),
        );

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


    fn get_files_to_parse() -> Vec<String> {
        let dir = "./french/";
        let mut files = crate::files::files_in_dir(dir).expect("Failed to read dir");
    
        for f in &mut files {
            f.insert_str(0, dir);
        }
        return files;
    }
}
