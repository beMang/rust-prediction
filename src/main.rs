use crate::app::App;

//We declare the different modules
pub mod app;
pub mod files;
pub mod predictiontree;
pub mod singlethreadparser;
pub mod string;

fn main() {
    let app = App::new();

    match app.run() {
        Err(e) => {
            println!("Erreur while running : {}", e.to_string())
        }
        _ => {}
    }
}
