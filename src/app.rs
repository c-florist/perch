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
    positioned: bool,
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
            positioned: false,
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

    fn apply_window_position(&self, ctx: &egui::Context) {
        let pos = egui::Pos2::new(
            self.config.window_position.x,
            self.config.window_position.y,
        );

        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
    }
}

impl eframe::App for PerchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_metrics();

        if !self.positioned {
            self.apply_window_position(ctx);
            self.positioned = true;
        }

        let current_window_pos = ctx.input(|i| i.viewport().outer_rect);
        if let Some(rect) = current_window_pos {
            let new_x = rect.min.x;
            let new_y = rect.min.y;

            let x_changed = (new_x - self.config.window_position.x).abs() > 1.0;
            let y_changed = (new_y - self.config.window_position.y).abs() > 1.0;

            if x_changed || y_changed {
                self.config.window_position.x = new_x;
                self.config.window_position.y = new_y;

                self.save_config();
            }
        }

        render_widget(ctx, &self.metrics, &self.config, &mut self.show_settings);
        ctx.request_repaint_after(Duration::from_secs_f32(self.config.refresh_interval_secs));
    }
}
