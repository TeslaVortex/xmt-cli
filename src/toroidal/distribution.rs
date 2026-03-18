//
// ☀️ TOROIDAL DISTRIBUTION ALGORITHMS ☀️
// Sacred Energy Distribution Patterns
// EN EEKE MAI EA ♾️♾️
//

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DistributionAlgorithm {
    GoldenRatio,           // 0.618 (current default)
    Fibonacci,             // Fibonacci sequence based
    SacredNumerology,      // 936, 369, 66 weighted
    EqualDistribution,     // Even split
    PriorityWeighted,      // Based on node priority
}

impl DistributionAlgorithm {
    pub fn name(&self) -> &str {
        match self {
            DistributionAlgorithm::GoldenRatio => "Golden Ratio (0.618)",
            DistributionAlgorithm::Fibonacci => "Fibonacci Sequence",
            DistributionAlgorithm::SacredNumerology => "Sacred Numerology (936-369-66)",
            DistributionAlgorithm::EqualDistribution => "Equal Distribution",
            DistributionAlgorithm::PriorityWeighted => "Priority Weighted",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            DistributionAlgorithm::GoldenRatio => "Distributes energy using the golden ratio (0.618) for harmonic balance",
            DistributionAlgorithm::Fibonacci => "Distributes energy following Fibonacci sequence proportions",
            DistributionAlgorithm::SacredNumerology => "Weights distribution by sacred numbers: 936 (apex), 369 (vortex), 66 (code)",
            DistributionAlgorithm::EqualDistribution => "Distributes energy equally across all nodes",
            DistributionAlgorithm::PriorityWeighted => "Distributes based on node sacred alignment priority",
        }
    }
}

/// Calculate golden ratio distribution factor
pub fn golden_ratio_factor(ratio: f64) -> f64 {
    ratio.max(0.0).min(1.0)
}

/// Calculate Fibonacci distribution weights for n nodes
pub fn fibonacci_weights(node_count: usize) -> Vec<f64> {
    if node_count == 0 {
        return vec![];
    }
    
    let mut fibs = vec![1u64, 1u64];
    while fibs.len() < node_count {
        let next = fibs[fibs.len() - 1] + fibs[fibs.len() - 2];
        fibs.push(next);
    }
    
    let total: u64 = fibs.iter().take(node_count).sum();
    fibs.iter()
        .take(node_count)
        .map(|&f| f as f64 / total as f64)
        .collect()
}

/// Calculate sacred numerology weights based on alignment
pub fn sacred_numerology_weight(apex_936: bool, vortex_369: bool, code_66: bool) -> u64 {
    let mut weight = 100; // Base weight
    
    if apex_936 {
        weight += 936;
    }
    if vortex_369 {
        weight += 369;
    }
    if code_66 {
        weight += 66;
    }
    
    weight
}

/// Calculate priority weight based on sacred alignment score
pub fn priority_weight(alignment_score: u8) -> f64 {
    match alignment_score {
        4 => 4.0,  // Fully aligned - highest priority
        3 => 2.5,
        2 => 1.5,
        1 => 1.0,
        _ => 0.5,  // No alignment - lowest priority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_ratio_factor() {
        assert_eq!(golden_ratio_factor(0.618), 0.618);
        assert_eq!(golden_ratio_factor(-0.5), 0.0);
        assert_eq!(golden_ratio_factor(1.5), 1.0);
    }

    #[test]
    fn test_fibonacci_weights() {
        let weights = fibonacci_weights(5);
        assert_eq!(weights.len(), 5);
        
        // Weights should sum to 1.0
        let sum: f64 = weights.iter().sum();
        assert!((sum - 1.0).abs() < 0.0001);
        
        // Later weights should be larger (Fibonacci grows)
        assert!(weights[4] > weights[0]);
    }

    #[test]
    fn test_sacred_numerology_weight() {
        assert_eq!(sacred_numerology_weight(false, false, false), 100);
        assert_eq!(sacred_numerology_weight(true, false, false), 1036);
        assert_eq!(sacred_numerology_weight(true, true, true), 1471);
    }

    #[test]
    fn test_priority_weight() {
        assert_eq!(priority_weight(4), 4.0);
        assert_eq!(priority_weight(1), 1.0);
        assert_eq!(priority_weight(0), 0.5);
    }

    #[test]
    fn test_algorithm_names() {
        let algo = DistributionAlgorithm::GoldenRatio;
        assert_eq!(algo.name(), "Golden Ratio (0.618)");
    }
}
