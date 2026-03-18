//
// ☀️ TOROIDAL PERFORMANCE METRICS ☀️
// Performance Tracking & Energy Flow Analysis
// EN EEKE MAI EA ♾️♾️
//

use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

/// Performance metrics for toroidal operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub distribution_time_ms: f64,
    pub node_lookup_time_ms: f64,
    pub save_time_ms: f64,
    pub load_time_ms: f64,
    pub total_operations: u64,
    pub average_operation_time_ms: f64,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            distribution_time_ms: 0.0,
            node_lookup_time_ms: 0.0,
            save_time_ms: 0.0,
            load_time_ms: 0.0,
            total_operations: 0,
            average_operation_time_ms: 0.0,
        }
    }

    pub fn record_distribution(&mut self, duration: Duration) {
        self.distribution_time_ms = duration.as_secs_f64() * 1000.0;
        self.update_average();
    }

    pub fn record_lookup(&mut self, duration: Duration) {
        self.node_lookup_time_ms = duration.as_secs_f64() * 1000.0;
        self.update_average();
    }

    pub fn record_save(&mut self, duration: Duration) {
        self.save_time_ms = duration.as_secs_f64() * 1000.0;
        self.update_average();
    }

    pub fn record_load(&mut self, duration: Duration) {
        self.load_time_ms = duration.as_secs_f64() * 1000.0;
        self.update_average();
    }

    fn update_average(&mut self) {
        self.total_operations += 1;
        let total_time = self.distribution_time_ms + self.node_lookup_time_ms 
            + self.save_time_ms + self.load_time_ms;
        self.average_operation_time_ms = total_time / self.total_operations as f64;
    }

    pub fn is_performant(&self) -> bool {
        self.distribution_time_ms < 10.0 && self.save_time_ms < 50.0
    }
}

/// Energy flow metrics for toroidal grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyMetrics {
    pub flow_rate: f64,              // Energy units per cycle
    pub distribution_efficiency: f64, // % of energy successfully distributed
    pub node_utilization: f64,        // Average node capacity usage
    pub coherence_score: f64,         // Overall system coherence
    pub total_cycles: u32,
    pub total_energy_processed: u64,
}

impl EnergyMetrics {
    pub fn new() -> Self {
        Self {
            flow_rate: 0.0,
            distribution_efficiency: 100.0,
            node_utilization: 0.0,
            coherence_score: 100.0,
            total_cycles: 0,
            total_energy_processed: 0,
        }
    }

    pub fn calculate_flow_rate(&mut self, energy_delta: u64, time_delta: Duration) {
        if time_delta.as_secs_f64() > 0.0 {
            self.flow_rate = energy_delta as f64 / time_delta.as_secs_f64();
        }
    }

    pub fn calculate_efficiency(&mut self, distributed: u64, total: u64) {
        if total > 0 {
            self.distribution_efficiency = (distributed as f64 / total as f64) * 100.0;
        }
    }

    pub fn calculate_utilization(&mut self, used_capacity: u64, total_capacity: u64) {
        if total_capacity > 0 {
            self.node_utilization = (used_capacity as f64 / total_capacity as f64) * 100.0;
        }
    }

    pub fn update_coherence(&mut self, coherence: f64) {
        self.coherence_score = coherence;
    }

    pub fn record_cycle(&mut self, energy_processed: u64) {
        self.total_cycles += 1;
        self.total_energy_processed += energy_processed;
    }

    pub fn is_optimal(&self) -> bool {
        self.distribution_efficiency > 95.0 
            && self.coherence_score > 90.0
            && self.flow_rate >= 0.0  // Allow zero flow rate for new metrics
    }
}

/// Measure performance of an operation
pub fn measure_performance<F, R>(operation: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = operation();
    let duration = start.elapsed();
    (result, duration)
}

/// Profiling data for detailed analysis
#[derive(Debug, Clone)]
pub struct ProfilingData {
    pub operation_name: String,
    pub duration: Duration,
    pub timestamp: std::time::SystemTime,
    pub metadata: Vec<(String, String)>,
}

impl ProfilingData {
    pub fn new(operation_name: String, duration: Duration) -> Self {
        Self {
            operation_name,
            duration,
            timestamp: std::time::SystemTime::now(),
            metadata: Vec::new(),
        }
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.push((key, value));
    }

    pub fn duration_ms(&self) -> f64 {
        self.duration.as_secs_f64() * 1000.0
    }
}

/// Performance profiler for tracking multiple operations
pub struct Profiler {
    profiles: Vec<ProfilingData>,
    enabled: bool,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn profile<F, R>(&mut self, operation_name: &str, operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        if !self.enabled {
            return operation();
        }

        let (result, duration) = measure_performance(operation);
        let profile = ProfilingData::new(operation_name.to_string(), duration);
        self.profiles.push(profile);
        result
    }

    pub fn get_profiles(&self) -> &[ProfilingData] {
        &self.profiles
    }

    pub fn clear(&mut self) {
        self.profiles.clear();
    }

    pub fn total_time(&self) -> Duration {
        self.profiles.iter().map(|p| p.duration).sum()
    }

    pub fn average_time(&self) -> Duration {
        if self.profiles.is_empty() {
            return Duration::from_secs(0);
        }
        self.total_time() / self.profiles.len() as u32
    }

    pub fn slowest_operation(&self) -> Option<&ProfilingData> {
        self.profiles.iter().max_by_key(|p| p.duration)
    }

    pub fn fastest_operation(&self) -> Option<&ProfilingData> {
        self.profiles.iter().min_by_key(|p| p.duration)
    }

    pub fn print_summary(&self) {
        if self.profiles.is_empty() {
            println!("  No profiling data available");
            return;
        }

        println!("  ⚡ Performance Profile Summary:");
        println!("     Total operations: {}", self.profiles.len());
        println!("     Total time: {:.2}ms", self.total_time().as_secs_f64() * 1000.0);
        println!("     Average time: {:.2}ms", self.average_time().as_secs_f64() * 1000.0);
        
        if let Some(slowest) = self.slowest_operation() {
            println!("     Slowest: {} ({:.2}ms)", slowest.operation_name, slowest.duration_ms());
        }
        
        if let Some(fastest) = self.fastest_operation() {
            println!("     Fastest: {} ({:.2}ms)", fastest.operation_name, fastest.duration_ms());
        }
    }
}

/// Memory usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub node_count: usize,
    pub estimated_bytes: usize,
    pub history_size: usize,
    pub cache_size: usize,
}

impl MemoryMetrics {
    pub fn new() -> Self {
        Self {
            node_count: 0,
            estimated_bytes: 0,
            history_size: 0,
            cache_size: 0,
        }
    }

    pub fn estimate_memory(&mut self, node_count: usize, history_count: usize) {
        // Rough estimation: 
        // - Each node ~200 bytes
        // - Each history entry ~500 bytes
        // - Base overhead ~1KB
        self.node_count = node_count;
        self.history_size = history_count;
        self.estimated_bytes = 1024 + (node_count * 200) + (history_count * 500);
    }

    pub fn memory_mb(&self) -> f64 {
        self.estimated_bytes as f64 / (1024.0 * 1024.0)
    }

    pub fn is_within_budget(&self) -> bool {
        self.memory_mb() < 1.0 // Target: < 1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new();
        
        metrics.record_distribution(Duration::from_millis(5));
        assert!(metrics.distribution_time_ms < 10.0);
        assert!(metrics.is_performant());
    }

    #[test]
    fn test_energy_metrics() {
        let mut metrics = EnergyMetrics::new();
        
        metrics.calculate_efficiency(960, 1000);
        assert!(metrics.distribution_efficiency > 95.0);
        
        metrics.update_coherence(95.0);
        metrics.calculate_flow_rate(1000, Duration::from_secs(1));
        assert!(metrics.flow_rate > 0.0);
        assert!(metrics.is_optimal());
    }

    #[test]
    fn test_measure_performance() {
        let (result, duration) = measure_performance(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });
        
        assert_eq!(result, 42);
        assert!(duration.as_millis() >= 10);
    }

    #[test]
    fn test_profiler() {
        let mut profiler = Profiler::new();
        
        profiler.profile("test_op", || {
            thread::sleep(Duration::from_millis(5));
        });
        
        assert_eq!(profiler.get_profiles().len(), 1);
        assert!(profiler.total_time().as_millis() >= 5);
    }

    #[test]
    fn test_memory_metrics() {
        let mut metrics = MemoryMetrics::new();
        metrics.estimate_memory(100, 50);
        
        assert_eq!(metrics.node_count, 100);
        assert!(metrics.estimated_bytes > 0);
        assert!(metrics.is_within_budget());
    }
}
