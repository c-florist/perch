use crate::models::SystemMetrics;
use crate::services::{format_bytes, SystemMonitor};
use std::time::{Duration, Instant};

pub struct PerchApp {
    monitor: SystemMonitor,
    metrics: SystemMetrics,
    last_update: Instant,
    refresh_interval: Duration,
}

impl PerchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let monitor = SystemMonitor::new();
        let metrics = SystemMetrics::zero();

        Self {
            monitor,
            metrics,
            last_update: Instant::now(),
            refresh_interval: Duration::from_secs(1),
        }
    }

    fn update_metrics(&mut self) {
        if self.last_update.elapsed() >= self.refresh_interval {
            self.metrics = self.monitor.collect();
            self.last_update = Instant::now();
        }
    }
}

impl eframe::App for PerchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_metrics();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("perch - System Monitor");
            ui.separator();

            ui.add_space(10.0);

            // CPU Metrics
            ui.label(
                egui::RichText::new("CPU")
                    .size(16.0)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 150, 255)),
            );
            ui.label(format!(
                "Usage: {:.1}%",
                self.metrics.cpu.usage_percentage
            ));

            let cores_to_show = self.metrics.cpu.per_core.iter().take(4);
            for (i, usage) in cores_to_show.enumerate() {
                ui.label(format!("Core {}: {:.1}%", i, usage));
            }

            ui.add_space(10.0);

            // Memory Metrics
            ui.label(
                egui::RichText::new("Memory")
                    .size(16.0)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 200, 100)),
            );
            ui.label(format!(
                "RAM: {} / {} ({:.1}%)",
                format_bytes(self.metrics.memory.used_bytes),
                format_bytes(self.metrics.memory.total_bytes),
                self.metrics.memory.usage_percentage()
            ));

            ui.label(format!(
                "Swap: {} / {} ({:.1}%)",
                format_bytes(self.metrics.memory.swap_used_bytes),
                format_bytes(self.metrics.memory.swap_total_bytes),
                self.metrics.memory.swap_percentage()
            ));

            ui.add_space(10.0);

            // Network Metrics
            ui.label(
                egui::RichText::new("Network")
                    .size(16.0)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 150, 100)),
            );
            ui.label(format!(
                "↓ Download: {}/s",
                format_bytes(self.metrics.network.received_rate as u64)
            ));
            ui.label(format!(
                "↑ Upload: {}/s",
                format_bytes(self.metrics.network.transmitted_rate as u64)
            ));
        });

        ctx.request_repaint_after(self.refresh_interval);
    }
}
