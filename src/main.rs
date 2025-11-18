mod app;
mod models;
mod services;

use app::PerchApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_title("perch - System Monitor"),
        ..Default::default()
    };

    eframe::run_native(
        "perch",
        native_options,
        Box::new(|cc| Ok(Box::new(PerchApp::new(cc)))),
    )
}
