mod app;
mod document;
mod file;
mod ui;

use app::NotepadApp;

use eframe::egui;

// fn start_app(height:& i32, width:)
//it is better to return a restul of eframe type so that I can know wheather the app
//started or not
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 650.0]),
        ..Default::default()
    };

    //the native opition is for the desktop application

    eframe::run_native(
        "Notepad",
        native_options,
        Box::new(|cc| Ok(Box::new(NotepadApp::new(cc)))),
    )
}
