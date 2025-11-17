use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WindowPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Custom { x: u32, y: u32 },
}

impl Default for WindowPosition {
    fn default() -> Self {
        Self::TopRight
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    Catppuccin,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}
