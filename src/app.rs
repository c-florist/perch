use crate::models::{SystemMetrics, UserConfig, WindowPosition};
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
        let screen_rect = ctx.input(|i| {
            i.viewport()
                .monitor_size
                .unwrap_or(egui::Vec2::new(1920.0, 1080.0))
        });

        let window_size = egui::Vec2::new(300.0, 450.0);

        let pos = match self.config.window {
            WindowPosition::TopLeft => egui::Pos2::new(20.0, 20.0),
            WindowPosition::TopRight => egui::Pos2::new(
                screen_rect.x - window_size.x - 20.0,
                20.0,
            ),
            WindowPosition::BottomLeft => egui::Pos2::new(
                20.0,
                screen_rect.y - window_size.y - 20.0,
            ),
            WindowPosition::BottomRight => egui::Pos2::new(
                screen_rect.x - window_size.x - 20.0,
                screen_rect.y - window_size.y - 20.0,
            ),
            WindowPosition::Custom { x, y } => {
                egui::Pos2::new(x as f32, y as f32)
            }
        };

        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
    }
}

impl eframe::App for PerchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_metrics();

        // if !self.positioned {
        //     self.apply_window_position(ctx);
        //     self.positioned = true;
        // }

        render_widget(ctx, &self.metrics, &self.config, &mut self.show_settings);
        ctx.request_repaint_after(Duration::from_secs_f32(self.config.refresh_interval_secs));
    }
}
