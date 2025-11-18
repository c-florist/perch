pub mod components;
pub mod theme;
pub mod widget;

use components::{metric_row, progress_bar_with_text, section_header, compact_text_bar, compact_percentage_bar, compact_metric};
pub use theme::ColorScheme;
pub use widget::render_widget;
