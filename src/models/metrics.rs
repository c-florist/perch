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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
}

impl MemoryMetrics {
    pub fn zero() -> Self {
        Self {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            swap_total_bytes: 0,
            swap_used_bytes: 0,
        }
    }

    pub fn usage_percentage(&self) -> f32 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f32 / self.total_bytes as f32) * 100.0
    }

    pub fn swap_percentage(&self) -> f32 {
        if self.swap_total_bytes == 0 {
            return 0.0;
        }
        (self.swap_used_bytes as f32 / self.swap_total_bytes as f32) * 100.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
    pub received_rate: f32,
    pub transmitted_rate: f32,
}

impl NetworkMetrics {
    pub fn zero() -> Self {
        Self {
            received_bytes: 0,
            transmitted_bytes: 0,
            received_rate: 0.0,
            transmitted_rate: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiskMetrics {
    pub read_bytes: u64,
    pub written_bytes: u64,
    pub read_rate: f32,
    pub write_rate: f32,
}

impl DiskMetrics {
    pub fn zero() -> Self {
        Self {
            read_bytes: 0,
            written_bytes: 0,
            read_rate: 0.0,
            write_rate: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu: CPUMetrics,
    pub memory: MemoryMetrics,
    pub network: NetworkMetrics,
    pub disk: DiskMetrics,
    pub timestamp: SystemTime,
}

impl SystemMetrics {
    pub fn zero() -> Self {
        Self {
            cpu: CPUMetrics::zero(),
            memory: MemoryMetrics::zero(),
            network: NetworkMetrics::zero(),
            disk: DiskMetrics::zero(),
            timestamp: SystemTime::now(),
        }
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

    #[test]
    fn test_memory_metrics_usage_percentage_calculates_correctly() {
        let metrics = MemoryMetrics {
            total_bytes: 1000,
            used_bytes: 750,
            available_bytes: 250,
            swap_total_bytes: 0,
            swap_used_bytes: 0,
        };
        assert_eq!(metrics.usage_percentage(), 75.0);
    }

    #[test]
    fn test_memory_metrics_swap_percentage_calculates_correctly() {
        let metrics = MemoryMetrics {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            swap_total_bytes: 2000,
            swap_used_bytes: 500,
        };
        assert_eq!(metrics.swap_percentage(), 25.0);
    }

    #[test]
    fn test_memory_metrics_percentage_zero_total() {
        let metrics = MemoryMetrics {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            swap_total_bytes: 0,
            swap_used_bytes: 0,
        };
        assert_eq!(metrics.usage_percentage(), 0.0);
        assert_eq!(metrics.swap_percentage(), 0.0);
    }
}
