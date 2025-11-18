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
    ui.add_space(6.0);
    ui.label(egui::RichText::new(text).strong().size(12.0));
    ui.separator();
    ui.add_space(2.0);
}

pub fn compact_metric(
    ui: &mut Ui,
    label: &str,
    value: &str,
    color: Color32,
) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .monospace()
                .color(Color32::GRAY)
                .size(11.0)
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .monospace()
                    .strong()
                    .color(color)
                    .size(11.0)
            );
        });
    });
}

pub fn compact_progress_bar(
    ui: &mut Ui,
    label: &str,
    fraction: f32,
    color: Color32,
) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .monospace()
                .size(11.0)
                .color(Color32::GRAY)
        );
        ui.add(
            ProgressBar::new(fraction)
                .fill(color)
                .desired_width(ui.available_width())
                .show_percentage()
                .animate(false)
        );
    });
}
