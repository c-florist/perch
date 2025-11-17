use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CPUMetrics {
    pub usage_percentage: f32,
    pub per_core: Vec<f32>,
}

impl CPUMetrics {
    pub fn zero() -> Self {
        Self {
            usage_percentage: 0.0,
            per_core: Vec::new(),
        }
    }

    pub fn formatted_usage(&self) -> String {
        format!("{:.2}%", self.usage_percentage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_metrics_formatted_usage_returns_correct_format() {
        let metrics = CPUMetrics {
            usage_percentage: 45.678,
            per_core: vec![],
        };
        assert_eq!(metrics.formatted_usage(), "45.68%");
    }
}
