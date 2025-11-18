use crate::models::{SystemMetrics, UserConfig};
use crate::services::{ConfigManager, SystemMonitor};
use std::time::{Duration, Instant};
use crate::ui::render_widget;

pub struct PerchApp {
    monitor: SystemMonitor,
    metrics: SystemMetrics,
    config: UserConfig,
    config_manager: ConfigManager,
    show_settings: bool,
    last_update: Instant,
}

impl PerchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config_manager = ConfigManager::new(
            ConfigManager::default_path().expect("Could not determine config path"),
        );
        let config = config_manager.load().unwrap_or_default();

        let monitor = SystemMonitor::new();
        let metrics = SystemMetrics::zero();

        Self {
            monitor,
            metrics,
            config,
            config_manager,
            show_settings: false,
            last_update: Instant::now(),
        }
    }

    fn update_metrics(&mut self) {
        let elapsed = self.last_update.elapsed();
        if elapsed >= Duration::from_secs_f32(self.config.refresh_interval_secs) {
            self.metrics = self.monitor.collect();
            self.last_update = Instant::now();
        }
    }

    fn save_config(&self) {
        if let Err(e) = self.config_manager.save(&self.config) {
            eprintln!("Failed to save config: {}", e);
        }
    }
}

impl eframe::App for PerchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_metrics();
        render_widget(ctx, &self.metrics, &self.config, &mut self.show_settings);
        ctx.request_repaint_after(Duration::from_secs_f32(self.config.refresh_interval_secs));
    }
}
