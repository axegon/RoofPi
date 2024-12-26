use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct CpuStat {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
}

impl CpuStat {
    fn total(&self) -> u64 {
        self.user
            + self.nice
            + self.system
            + self.idle
            + self.iowait
            + self.irq
            + self.softirq
            + self.steal
            + self.guest
            + self.guest_nice
    }
}

pub(crate) struct SystemResources;

impl SystemResources {
    pub fn new() -> Self {
        SystemResources
    }

    /// Read the CPU stats from /proc/stat
    /// # Returns
    /// A vector of tuples containing the CPU label and the CPU stats
    fn read_cpu_stats(&self) -> Vec<(String, CpuStat)> {
        let content = fs::read_to_string("/proc/stat").unwrap_or_default();
        let mut stats = Vec::new();

        for line in content.lines() {
            if line.starts_with("cpu") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let label = parts[0].to_string();

                if parts.len() < 11 {
                    continue;
                }

                let user = parts[1].parse().unwrap_or(0);
                let nice = parts[2].parse().unwrap_or(0);
                let system = parts[3].parse().unwrap_or(0);
                let idle = parts[4].parse().unwrap_or(0);
                let iowait = parts[5].parse().unwrap_or(0);
                let irq = parts[6].parse().unwrap_or(0);
                let softirq = parts[7].parse().unwrap_or(0);
                let steal = parts[8].parse().unwrap_or(0);
                let guest = parts[9].parse().unwrap_or(0);
                let guest_nice = parts[10].parse().unwrap_or(0);

                let stat = CpuStat {
                    user,
                    nice,
                    system,
                    idle,
                    iowait,
                    irq,
                    softirq,
                    steal,
                    guest,
                    guest_nice,
                };

                stats.push((label, stat));
            }
        }
        stats
    }

    fn get_cpu_stats(&self) -> usize {
        let stat1 = self.read_cpu_stats();
        thread::sleep(Duration::from_millis(500));

        let stat2 = self.read_cpu_stats();
        let mut max_usage_ratio = 0.0;

        for (label1, s1) in &stat1 {
            if let Some((_, s2)) = stat2.iter().find(|(label2, _)| label2 == label1) {
                let total_diff = s2.total().saturating_sub(s1.total());
                let idle_diff = (s2.idle + s2.iowait).saturating_sub(s1.idle + s1.iowait);

                if total_diff > 0 {
                    let usage_ratio = (total_diff - idle_diff) as f64 / total_diff as f64;
                    if usage_ratio > max_usage_ratio {
                        max_usage_ratio = usage_ratio;
                    }
                }
            }
        }

        let usage_scaled = (max_usage_ratio * 10.0).round() as usize;
        usage_scaled.clamp(0, 10)
    }

    /// Get a string representation of the CPU usage for the 16x2 display.
    /// # Returns
    /// A string with 15 characters, representing the CPU usage.
    pub fn get_line(&self) -> String {
        let cpu = self.get_cpu_stats();
        let filled = "#".repeat(cpu);
        let empty = " ".repeat(10 - cpu);

        format!("CPU: {}{}", filled, empty)
    }
}
