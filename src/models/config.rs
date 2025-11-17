use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WindowPosition {
    TopLeft,
    #[default]
    TopRight,
    BottomLeft,
    BottomRight,
    Custom {
        x: u32,
        y: u32,
    },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Dark,
    Light,
    Catppuccin,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserConfig {
    pub window: WindowPosition,
    pub transparency: f32,
    pub refresh_interval_secs: f32,
    pub always_on_top: bool,
    pub show_cpu_per_core: bool,
    pub theme: Theme,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            window: WindowPosition::default(),
            transparency: 0.8,
            refresh_interval_secs: 1.0,
            always_on_top: true,
            show_cpu_per_core: false,
            theme: Theme::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_user_config_holds_expected_values() {
        let config = UserConfig::default();
        assert_eq!(config.transparency, 0.8);
        assert_eq!(config.refresh_interval_secs, 1.0);
        assert!(config.always_on_top);
    }

    #[test]
    fn test_window_position_default_is_top_right() {
        let position = WindowPosition::default();
        assert_eq!(position, WindowPosition::TopRight);
    }
}
