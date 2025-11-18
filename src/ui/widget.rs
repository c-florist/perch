use crate::models::{SystemMetrics, UserConfig};
use crate::services::{format_bytes, format_rate};
use crate::ui::{ColorScheme, metric_row, progress_bar_with_text, section_header};
use egui::{Context, Ui, Color32};
use crate::ui::components::{compact_metric, compact_progress_bar};

pub fn render_widget(
    ctx: &Context,
    metrics: &SystemMetrics,
    config: &UserConfig,
    show_settings: &mut bool,
) {
    let colors = ColorScheme::from_theme(config.theme);

    egui::CentralPanel::default()
        .frame(
            egui::Frame::default()
                .fill(egui::Color32::TRANSPARENT)
                .corner_radius(8.0)
                .inner_margin(8.0)
                .shadow(egui::epaint::Shadow {
                    offset: [0, 2],
                    blur: 8,
                    spread: 0,
                    color: egui::Color32::from_black_alpha(80),
                }),
        )
        .show(ctx, |ui| {
            let panel_response = ui.interact(
                ui.max_rect(),
                ui.id().with("drag_area"),
                egui::Sense::click_and_drag(),
            );

            let is_hovering = panel_response.hovered();

            let transparency = if is_hovering {
                (config.transparency + 0.3).min(1.0)
            } else {
                config.transparency
            };

            let bg_color = colors.background.linear_multiply(transparency);

            ui.painter().rect_filled(
                ui.max_rect(),
                8.0,
                bg_color,
            );

            if panel_response.drag_started() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }

            render_header(ui, &colors, show_settings);
            ui.add_space(4.0);
            render_cpu(ui, &metrics.cpu, &colors, config);
            render_memory(ui, &metrics.memory, &colors);
            ui.add_space(4.0);
            render_network_and_disk(ui, &metrics.network, &metrics.disk, &colors);
        });
}

fn render_header(ui: &mut Ui, colors: &ColorScheme, show_settings: &mut bool) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("perch")
                .size(12.0)
                .strong()
                .color(colors.primary),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .small_button(egui::RichText::new("⚙").size(14.0))
                .on_hover_text("Settings")
                .clicked()
            {
                *show_settings = !*show_settings;
            }
        });
    });
    ui.separator();
}

fn render_cpu(ui: &mut Ui, cpu: &crate::models::CPUMetrics, colors: &ColorScheme, config: &UserConfig) {
    let cpu_color = colors.cpu_color(cpu.usage_percentage);
    compact_progress_bar(
        ui,
        "CPU",
        cpu.usage_percentage / 100.0,
        cpu_color,
    );

    if config.show_cpu_per_core && !cpu.per_core.is_empty() {
        ui.add_space(2.0);
        ui.collapsing("cores", |ui| {
            for (i, core_usage) in cpu.per_core.iter().enumerate() {
                let core_color = colors.cpu_color(*core_usage);
                compact_progress_bar(
                    ui,
                    &format!("C{}", i),
                    *core_usage / 100.0,
                    core_color,
                );
            }
        });
    }
}

fn render_memory(ui: &mut Ui, memory: &crate::models::MemoryMetrics, colors: &ColorScheme) {
    ui.add_space(4.0);

    let mem_usage = memory.usage_percentage();
    let mem_color = colors.memory_color(mem_usage);

    compact_progress_bar(
        ui,
        "MEM",
        mem_usage / 100.0,
        mem_color,
    );

    if memory.swap_total_bytes > 0 {
        let swap_usage = memory.swap_percentage();
        let swap_color = colors.memory_color(swap_usage);
        compact_progress_bar(
            ui,
            "SWP",
            swap_usage / 100.0,
            swap_color,
        );
    }
}

fn render_network_and_disk(
    ui: &mut Ui,
    network: &crate::models::NetworkMetrics,
    disk: &crate::models::DiskMetrics,
    colors: &ColorScheme,
) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("↓")
                .monospace()
                .color(Color32::GRAY)
                .size(11.0)
        );
        ui.label(
            egui::RichText::new(format_rate(network.received_rate as u64))
                .monospace()
                .color(colors.primary)
                .size(11.0)
        );

        ui.add_space(8.0);  // Spacer

        ui.label(
            egui::RichText::new("↑")
                .monospace()
                .color(Color32::GRAY)
                .size(11.0)
        );
        ui.label(
            egui::RichText::new(format_rate(network.transmitted_rate as u64))
                .monospace()
                .color(colors.secondary)
                .size(11.0)
        );
    });

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("R")
                .monospace()
                .color(Color32::GRAY)
                .size(11.0)
        );
        ui.label(
            egui::RichText::new(format_rate(disk.read_rate as u64))
                .monospace()
                .color(colors.success)
                .size(11.0)
        );

        ui.add_space(8.0);

        ui.label(
            egui::RichText::new("W")
                .monospace()
                .color(Color32::GRAY)
                .size(11.0)
        );
        ui.label(
            egui::RichText::new(format_rate(disk.write_rate as u64))
                .monospace()
                .color(colors.warning)
                .size(11.0)
        );
    });
}
