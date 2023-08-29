use std::{
    fmt,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use egui::{Color32, RichText, Ui};

use crate::{
    predictiontree::{nice_print_of_possibilities, PredictionTree},
    singlethreadparser,
    string::last_words,
};

/**
 * Application state
 */
pub enum State {
    Loading(JoinHandle<()>),
    Ready,
    Starting,
    Exit,
}

impl State {
    fn new() -> State {
        State::Starting
    }

    pub fn ready(&mut self) {
        *self = State::Ready;
    }

    pub fn loading(&mut self, handle: JoinHandle<()>) {
        *self = State::Loading(handle);
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
            State::Loading(_handle) => "Loading...",
            State::Exit => "Exiting...",
            State::Ready => "Ready !",
            State::Starting => "Starting...",
        };
        write!(f, "{}", value)
    }
}

pub struct App {
    prediction_tree: Arc<Mutex<PredictionTree>>,
    state: State,
    ouput: String,
    input: String,
}

impl App {
    pub fn new() -> App {
        return App {
            prediction_tree: Arc::new(Mutex::new(PredictionTree::new_empty())),
            state: State::new(),
            input: String::new(),
            ouput: String::new(),
        };
    }

    /**
     * Fonction chargée de charger la base de donnée
     */
    fn load_sets(&mut self) {
        self.prediction_tree = Arc::new(Mutex::new(PredictionTree::new_empty()));

        let prediction_tree = Arc::clone(&self.prediction_tree);
        self.state.loading(thread::spawn(move || {
            let mut prediction = prediction_tree.lock().unwrap();
            *prediction =
                singlethreadparser::load_tree_from_files(Self::get_files_to_parse("./french/"));
            singlethreadparser::add_files_to_tree(
                Self::get_files_to_parse("./tweets/"),
                &mut prediction,
            );
        }));
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
                    //we could try to make an automated predition :)
                }
            }
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
            let last_words = last_words(&self.input, 2).unwrap(); //On récupère les 2 derniers mots
            match self
                .prediction_tree
                .lock()
                .unwrap()
                .retrieve_possibilities(last_words.get(0).unwrap(), last_words.get(1).unwrap())
            {
                None => {
                    self.ouput = "No result for thoses 2 words".to_string();
                }
                Some(possibilities) => self.ouput = nice_print_of_possibilities(possibilities),
            }
        }
    }

    fn build_top_pannel(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label(format!("State : {}", self.state.to_string()));
        });
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &mut self.state {
            State::Loading(handle) => {
                if handle.is_finished() {
                    self.state.ready();
                } else {
                    println!("hasnt finished yet {}", self.state.to_string());
                }
            }
            State::Starting => self.load_sets(),
            _ => {} //everything is ok we do nothing
        };

        custom_window_frame(
            ctx,
            frame,
            "rust-prediction, a better tool than twitozz",
            |ui| {
                self.build_top_pannel(ui);
                self.build_central_panel(ui);
            },
        );
    }
}

/*
 *  ************************************************************
 *  *************** WINDOW COUNTOUR MANAGEMENT******************
 *  ************************************************************
 */

fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, frame, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str,
) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui, frame);
        });
    });
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    use egui::Button;

    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        frame.close();
    }

    if frame.info().window_info.maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            frame.set_maximized(false);
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            frame.set_maximized(true);
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        frame.set_minimized(true);
    }
}
