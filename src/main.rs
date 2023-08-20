use crate::app::App;

//We declare the different modules
pub mod app;
pub mod files;
pub mod predictiontree;
pub mod singlethreadparser;
pub mod string;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.resizable = true;
    options.drag_and_drop_support = true;
    options.decorated = true;

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
