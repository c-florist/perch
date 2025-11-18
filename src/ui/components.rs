use egui::{Color32, ProgressBar, Response, Ui};

pub fn metric_row(
    ui: &mut Ui,
    label: &str,
    value: &str,
    color: Color32,
) -> Response {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).monospace().color(Color32::GRAY));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(value).monospace().strong().color(color));
        });
    })
        .response
}

pub fn progress_bar_with_text(
    ui: &mut Ui,
    label: &str,
    fraction: f32,
    text: &str,
    color: Color32,
) {
    ui.label(egui::RichText::new(label).monospace().small());
    ui.add(
        ProgressBar::new(fraction)
            .fill(color)
            .text(text)
            .animate(false),
    );
}

pub fn section_header(ui: &mut Ui, text: &str) {
    ui.add_space(8.0);
    ui.label(egui::RichText::new(text).strong().size(14.0));
    ui.separator();
    ui.add_space(4.0);
}
