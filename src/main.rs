mod app;
mod models;
mod services;
mod ui;

use app::PerchApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([200.0, 160.0])
            .with_position([100.0, 100.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(true)
            .with_title("perch"),
        ..Default::default()
    };

    eframe::run_native(
        "perch",
        native_options,
        Box::new(|cc| Ok(Box::new(PerchApp::new(cc)))),
    )
}
