mod grid;
mod my_app;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(1024.0, 1024.0)),
        ..Default::default()
    };
    
    eframe::run_native("Game of Life",
     options,
     Box::new(|_cc| Box::<my_app::MyApp>::default()),
    )
}
