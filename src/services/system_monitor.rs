use crate::models::{CPUMetrics, DiskMetrics, MemoryMetrics, NetworkMetrics, SystemMetrics};
use std::time::SystemTime;
use sysinfo::{Networks, System};

pub struct SystemMonitor {
    system: System,
    networks: Networks,
    previous_network: (u64, u64),
    previous_disk: (u64, u64),
    last_update: SystemTime,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let networks = Networks::new_with_refreshed_list();

        Self {
            system,
            networks,
            previous_network: (0, 0),
            previous_disk: (0, 0),
            last_update: SystemTime::now(),
        }
    }

    pub fn collect(&mut self) -> SystemMetrics {
        self.system.refresh_all();
        self.networks.refresh(true);
        let now = SystemTime::now();

        let cpu = self.collect_cpu_metrics();
        let memory = self.collect_memory_metrics();
        let network = self.collect_network_metrics(now);
        let disk = self.collect_disk_metrics(now);

        self.last_update = now;

        SystemMetrics {
            cpu,
            memory,
            network,
            disk,
            timestamp: now,
        }
    }

    fn collect_cpu_metrics(&self) -> CPUMetrics {
        let usage_percentage = self.system.global_cpu_usage();

        let per_core: Vec<f32> = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        CPUMetrics {
            usage_percentage,
            per_core,
        }
    }

    fn collect_memory_metrics(&self) -> MemoryMetrics {
        MemoryMetrics {
            total_bytes: self.system.total_memory(),
            used_bytes: self.system.used_memory(),
            available_bytes: self.system.available_memory(),
            swap_total_bytes: self.system.total_swap(),
            swap_used_bytes: self.system.used_swap(),
        }
    }

    fn collect_network_metrics(&mut self, now: SystemTime) -> NetworkMetrics {
        let mut total_received = 0u64;
        let mut total_transmitted = 0u64;

        // Sum up all network interfaces
        for (_interface_name, data) in &self.networks {
            total_received += data.total_received();
            total_transmitted += data.total_transmitted();
        }

        let time_delta = now
            .duration_since(self.last_update)
            .unwrap_or(std::time::Duration::from_secs(1))
            .as_secs_f64();

        let received_rate = if time_delta > 0.0 {
            ((total_received.saturating_sub(self.previous_network.0) as f64) / time_delta) as f32
        } else {
            0.0
        };

        let transmitted_rate = if time_delta > 0.0 {
            ((total_transmitted.saturating_sub(self.previous_network.1) as f64) / time_delta) as f32
        } else {
            0.0
        };

        self.previous_network = (total_received, total_transmitted);

        NetworkMetrics {
            received_bytes: total_received,
            transmitted_bytes: total_transmitted,
            received_rate,
            transmitted_rate,
        }
    }

    fn collect_disk_metrics(&mut self, now: SystemTime) -> DiskMetrics {
        let total_read = 0u64;
        let total_written = 0u64;

        let time_delta = now
            .duration_since(self.last_update)
            .unwrap_or(std::time::Duration::from_secs(1))
            .as_secs_f64();

        let read_rate = if time_delta > 0.0 {
            (total_read.saturating_sub(self.previous_disk.0) as f64 / time_delta) as f32
        } else {
            0.0
        };

        let write_rate = if time_delta > 0.0 {
            (total_written.saturating_sub(self.previous_disk.1) as f64 / time_delta) as f32
        } else {
            0.0
        };

        self.previous_disk = (total_read, total_written);

        DiskMetrics {
            read_bytes: total_read,
            written_bytes: total_written,
            read_rate,
            write_rate,
        }
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new_system_monitor() {
        let monitor = SystemMonitor::new();
        assert_eq!(monitor.previous_network, (0, 0));
        assert_eq!(monitor.previous_disk, (0, 0));
    }

    #[test]
    fn test_cpu_metrics_in_valid_range() {
        let mut monitor = SystemMonitor::new();
        std::thread::sleep(Duration::from_millis(100));
        let metrics = monitor.collect();

        assert!(metrics.cpu.usage_percentage >= 0.0);
        assert!(metrics.cpu.usage_percentage <= 100.0);
        assert!(!metrics.cpu.per_core.is_empty());
    }

    #[test]
    fn test_memory_metrics_are_valid() {
        let mut monitor = SystemMonitor::new();
        let metrics = monitor.collect();

        assert!(metrics.memory.total_bytes > 0);
        assert!(metrics.memory.used_bytes <= metrics.memory.total_bytes);
    }

    #[test]
    fn test_network_bytes_are_cumulative() {
        let mut monitor = SystemMonitor::new();

        let first = monitor.collect();
        std::thread::sleep(Duration::from_secs(1));
        let second = monitor.collect();

        assert!(second.network.received_bytes >= first.network.received_bytes);
        assert!(second.network.transmitted_bytes >= first.network.transmitted_bytes);
    }

    #[test]
    fn test_multiple_collections() {
        let mut monitor = SystemMonitor::new();

        let first = monitor.collect();
        std::thread::sleep(Duration::from_secs(1));
        let second = monitor.collect();

        assert_ne!(first.timestamp, second.timestamp);
    }
}
