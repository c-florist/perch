use crate::models::Theme;
use egui::Color32;

pub struct ColorScheme {
    pub background: Color32,
    pub surface: Color32,
    pub primary: Color32,
    pub secondary: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
}

impl ColorScheme {
    pub fn from_theme(theme: Theme) -> Self {
        match theme {
            Theme::Dark => Self::dark(),
            Theme::Light => Self::light(),
            Theme::Catppuccin => Self::catppuccin(),
        }
    }

    fn dark() -> Self {
        Self {
            background: Color32::from_rgb(15, 15, 20),
            surface: Color32::from_rgb(25, 25, 30),
            primary: Color32::from_rgb(100, 150, 255),
            secondary: Color32::from_rgb(150, 100, 255),
            text: Color32::from_rgb(240, 240, 245),
            text_dim: Color32::from_rgb(160, 160, 170),
            success: Color32::from_rgb(80, 200, 120),
            warning: Color32::from_rgb(255, 180, 50),
            error: Color32::from_rgb(255, 100, 100),
        }
    }

    fn light() -> Self {
        Self {
            background: Color32::from_rgb(250, 250, 255),
            surface: Color32::from_rgb(240, 240, 245),
            primary: Color32::from_rgb(50, 100, 200),
            secondary: Color32::from_rgb(100, 50, 200),
            text: Color32::from_rgb(20, 20, 25),
            text_dim: Color32::from_rgb(100, 100, 110),
            success: Color32::from_rgb(40, 150, 80),
            warning: Color32::from_rgb(200, 130, 20),
            error: Color32::from_rgb(200, 50, 50),
        }
    }

    fn catppuccin() -> Self {
        Self {
            background: Color32::from_rgb(30, 30, 46),
            surface: Color32::from_rgb(49, 50, 68),
            primary: Color32::from_rgb(137, 180, 250),
            secondary: Color32::from_rgb(203, 166, 247),
            text: Color32::from_rgb(205, 214, 244),
            text_dim: Color32::from_rgb(147, 153, 178),
            success: Color32::from_rgb(166, 227, 161),
            warning: Color32::from_rgb(249, 226, 175),
            error: Color32::from_rgb(243, 139, 168),
        }
    }

    pub fn cpu_color(&self, usage: f32) -> Color32 {
        if usage > 80.0 {
            self.error
        } else if usage > 50.0 {
            self.warning
        } else {
            self.success
        }
    }

    pub fn memory_color(&self, usage: f32) -> Color32 {
        if usage > 85.0 {
            self.error
        } else if usage > 70.0 {
            self.warning
        } else {
            self.success
        }
    }
}
