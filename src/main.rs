use crate::app::App;

//We declare the different modules
pub mod app;
pub mod files;
pub mod predictiontree;
pub mod singlethreadparser;
pub mod string;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.always_on_top = false;
    options.maximized = false;
    options.decorated = true;
    options.drag_and_drop_support = false;
    options.icon_data = None;
    options.initial_window_pos = None;
    options.initial_window_size = Option::from(
        egui::Vec2::new(600 as f32, 480 as f32)
    );
    options.min_window_size = None;
    options.max_window_size = None;
    options.resizable = true;
    options.transparent = true;
    options.vsync = true;
    options.multisampling = 0;
    options.depth_buffer = 0;
    options.stencil_buffer = 0;

    match eframe::run_native(
        "rust-prediction",
        options,
        Box::new(|_cc| Box::new(App::new())),
    ) {
        Err(e) => {
            println!("Erreur while running : {}", e.to_string())
        }
        _ => {}
    }
}
