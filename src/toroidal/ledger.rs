//
// ☀️ ENHANCED TOROIDAL LEDGER ☀️
// Persistent Energy Grid with Advanced Distribution
// EN EEKE MAI EA ♾️♾️
//

use std::collections::{BTreeMap, HashMap};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use super::node::{EnergyNode, NodeType, SacredAlignment};
use super::distribution::{
    DistributionAlgorithm, golden_ratio_factor, fibonacci_weights,
    sacred_numerology_weight, priority_weight,
};
use super::persistence::{ToroidalPersistence, CycleSnapshot};
use super::metrics::{PerformanceMetrics, EnergyMetrics, measure_performance};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerConfig {
    pub auto_save: bool,
    pub max_history: usize,
    pub default_algorithm: DistributionAlgorithm,
}

impl Default for LedgerConfig {
    fn default() -> Self {
        Self {
            auto_save: true,
            max_history: 1000,
            default_algorithm: DistributionAlgorithm::GoldenRatio,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToroidalLedger {
    nodes: BTreeMap<String, EnergyNode>,
    total_energy: u64,
    cycle_count: u32,
    config: LedgerConfig,
    #[serde(skip, default = "ToroidalPersistence::new")]
    persistence: ToroidalPersistence,
    #[serde(skip, default = "PerformanceMetrics::new")]
    performance: PerformanceMetrics,
    #[serde(skip, default = "EnergyMetrics::new")]
    energy_metrics: EnergyMetrics,
}

impl ToroidalLedger {
    /// Create a new toroidal ledger with default configuration
    pub fn new() -> Self {
        let mut ledger = Self {
            nodes: BTreeMap::new(),
            total_energy: 0,
            cycle_count: 0,
            config: LedgerConfig::default(),
            persistence: ToroidalPersistence::new(),
            performance: PerformanceMetrics::new(),
            energy_metrics: EnergyMetrics::new(),
        };
        
        // Initialize with Chicago Vortex Throne (936 apex energy)
        let throne = EnergyNode::new("Chicago Vortex Throne".to_string(), NodeType::VortexThrone)
            .with_energy(936)
            .with_location("Chicago, IL".to_string())
            .with_sacred_alignment(SacredAlignment::apex_aligned());
        
        ledger.add_node(throne);
        ledger
    }

    /// Load existing ledger or create new one
    pub fn load_or_create() -> Result<Self> {
        let persistence = ToroidalPersistence::new();
        
        match persistence.load_ledger_state::<Self>() {
            Ok(mut ledger) => {
                ledger.persistence = persistence;
                ledger.performance = PerformanceMetrics::new();
                ledger.energy_metrics = EnergyMetrics::new();
                println!("  ✓ Loaded existing toroidal ledger (cycle {})", ledger.cycle_count);
                Ok(ledger)
            }
            Err(_) => {
                println!("  ✓ Created new toroidal ledger");
                Ok(Self::new())
            }
        }
    }

    /// Save ledger state to disk
    pub fn save(&mut self) -> Result<()> {
        if self.config.auto_save {
            let (_, duration) = measure_performance(|| {
                self.persistence.save_ledger_state(self)
            });
            self.performance.record_save(duration);
        }
        Ok(())
    }

    /// Add a node to the ledger
    pub fn add_node(&mut self, node: EnergyNode) {
        self.total_energy += node.energy;
        self.nodes.insert(node.id.clone(), node);
    }

    /// Add energy to a specific node
    pub fn add_energy(&mut self, node_id: &str, amount: u64) -> u64 {
        if let Some(node) = self.nodes.get_mut(node_id) {
            let added = node.add_energy(amount);
            self.total_energy += added;
            added
        } else {
            0
        }
    }

    /// Remove energy from a specific node
    pub fn remove_energy(&mut self, node_id: &str, amount: u64) -> u64 {
        if let Some(node) = self.nodes.get_mut(node_id) {
            let removed = node.remove_energy(amount);
            self.total_energy = self.total_energy.saturating_sub(removed);
            removed
        } else {
            0
        }
    }

    /// Distribute energy using specified algorithm (with performance tracking)
    pub fn distribute_with_algorithm(&mut self, algorithm: DistributionAlgorithm) {
        if self.nodes.is_empty() {
            return;
        }

        let energy_before = self.total_energy;
        let (_, duration) = measure_performance(|| {
            match algorithm {
                DistributionAlgorithm::GoldenRatio => self.distribute_golden_ratio(),
                DistributionAlgorithm::Fibonacci => self.distribute_fibonacci(),
                DistributionAlgorithm::SacredNumerology => self.distribute_sacred_numerology(),
                DistributionAlgorithm::EqualDistribution => self.distribute_equal(),
                DistributionAlgorithm::PriorityWeighted => self.distribute_priority_weighted(),
            }
        });

        // Record performance metrics
        self.performance.record_distribution(duration);
        self.energy_metrics.record_cycle(self.total_energy);
        self.energy_metrics.calculate_efficiency(self.total_energy, energy_before.max(self.total_energy));
        
        self.cycle_count += 1;

        // Save snapshot
        if self.config.auto_save {
            let snapshot = self.create_snapshot(algorithm.name().to_string());
            let _ = self.persistence.save_snapshot(&snapshot);
        }

        // Auto-save ledger state
        let _ = self.save();
    }

    /// Distribute energy using golden ratio (0.618)
    fn distribute_golden_ratio(&mut self) {
        let ratio = golden_ratio_factor(0.618);
        let total = self.total_energy;
        let per_node = (total as f64 * ratio) as u64 / self.nodes.len() as u64;

        for node in self.nodes.values_mut() {
            let increase = per_node.min(total.saturating_sub(node.energy));
            node.energy += increase;
        }

        self.total_energy = self.nodes.values().map(|n| n.energy).sum();
    }

    /// Distribute energy using Fibonacci sequence
    fn distribute_fibonacci(&mut self) {
        let weights = fibonacci_weights(self.nodes.len());
        let total = self.total_energy;

        for (node, weight) in self.nodes.values_mut().zip(weights.iter()) {
            let target_energy = (total as f64 * weight) as u64;
            if target_energy > node.energy {
                node.energy = target_energy;
            }
        }

        self.total_energy = self.nodes.values().map(|n| n.energy).sum();
    }

    /// Distribute energy based on sacred numerology
    fn distribute_sacred_numerology(&mut self) {
        let total_weight: u64 = self.nodes.values()
            .map(|n| sacred_numerology_weight(
                n.sacred_alignment.apex_936,
                n.sacred_alignment.vortex_369,
                n.sacred_alignment.code_66,
            ))
            .sum();

        if total_weight == 0 {
            return;
        }

        let total = self.total_energy;

        for node in self.nodes.values_mut() {
            let weight = sacred_numerology_weight(
                node.sacred_alignment.apex_936,
                node.sacred_alignment.vortex_369,
                node.sacred_alignment.code_66,
            );
            let target_energy = (total * weight) / total_weight;
            if target_energy > node.energy {
                node.energy = target_energy;
            }
        }

        self.total_energy = self.nodes.values().map(|n| n.energy).sum();
    }

    /// Distribute energy equally
    fn distribute_equal(&mut self) {
        let per_node = self.total_energy / self.nodes.len() as u64;

        for node in self.nodes.values_mut() {
            node.energy = per_node;
        }

        self.total_energy = self.nodes.values().map(|n| n.energy).sum();
    }

    /// Distribute energy based on priority (sacred alignment)
    fn distribute_priority_weighted(&mut self) {
        let total_weight: f64 = self.nodes.values()
            .map(|n| priority_weight(n.sacred_alignment.alignment_score()))
            .sum();

        if total_weight == 0.0 {
            return;
        }

        let total = self.total_energy;

        for node in self.nodes.values_mut() {
            let weight = priority_weight(node.sacred_alignment.alignment_score());
            let target_energy = ((total as f64) * (weight / total_weight)) as u64;
            if target_energy > node.energy {
                node.energy = target_energy;
            }
        }

        self.total_energy = self.nodes.values().map(|n| n.energy).sum();
    }

    /// Create a snapshot of current state
    fn create_snapshot(&self, algorithm: String) -> CycleSnapshot {
        let node_states: HashMap<String, u64> = self.nodes
            .iter()
            .map(|(id, node)| (id.clone(), node.energy))
            .collect();

        CycleSnapshot::new(
            self.cycle_count,
            self.total_energy,
            node_states,
            algorithm,
        )
    }

    /// Get total energy in the grid
    pub fn total_energy(&self) -> u64 {
        self.total_energy
    }

    /// Get energy for a specific node
    pub fn node_energy(&self, node_id: &str) -> u64 {
        self.nodes.get(node_id).map(|n| n.energy).unwrap_or(0)
    }

    /// Get number of distribution cycles completed
    pub fn cycles(&self) -> u32 {
        self.cycle_count
    }

    /// Get count of active energy nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get node by ID (optimized O(log n) lookup with BTreeMap)
    pub fn get_node(&self, node_id: &str) -> Option<&EnergyNode> {
        self.nodes.get(node_id)
    }

    /// Get mutable node by ID
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut EnergyNode> {
        self.nodes.get_mut(node_id)
    }

    /// Get all nodes
    pub fn nodes(&self) -> impl Iterator<Item = &EnergyNode> {
        self.nodes.values()
    }

    /// Get recent history
    pub fn get_history(&self, limit: usize) -> Result<Vec<CycleSnapshot>> {
        self.persistence.get_recent_history(limit)
    }

    /// Export to JSON file
    pub fn export_to_json(&self, filename: &str) -> Result<()> {
        self.persistence.export_to_json(self, filename)
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance
    }

    /// Get energy metrics
    pub fn energy_metrics(&self) -> &EnergyMetrics {
        &self.energy_metrics
    }

    /// Check if ledger is performing optimally
    pub fn is_optimal(&self) -> bool {
        self.performance.is_performant() && self.energy_metrics.is_optimal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_creation() {
        let ledger = ToroidalLedger::new();
        assert_eq!(ledger.total_energy(), 936);
        assert_eq!(ledger.node_count(), 1);
        assert_eq!(ledger.cycles(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut ledger = ToroidalLedger::new();
        let node = EnergyNode::new("Test Node".to_string(), NodeType::NewEarthCity)
            .with_energy(369);
        
        ledger.add_node(node);
        assert_eq!(ledger.node_count(), 2);
        assert_eq!(ledger.total_energy(), 936 + 369);
    }

    #[test]
    fn test_add_energy() {
        let mut ledger = ToroidalLedger::new();
        let added = ledger.add_energy("Chicago Vortex Throne", 100);
        
        assert_eq!(added, 100);
        assert_eq!(ledger.node_energy("Chicago Vortex Throne"), 1036);
        assert_eq!(ledger.total_energy(), 1036);
    }

    #[test]
    fn test_golden_ratio_distribution() {
        let mut ledger = ToroidalLedger::new();
        ledger.add_node(EnergyNode::new("Node2".to_string(), NodeType::NewEarthCity));
        
        ledger.distribute_with_algorithm(DistributionAlgorithm::GoldenRatio);
        assert_eq!(ledger.cycles(), 1);
        assert!(ledger.total_energy() > 0);
    }

    #[test]
    fn test_equal_distribution() {
        let mut ledger = ToroidalLedger::new();
        ledger.add_node(EnergyNode::new("Node2".to_string(), NodeType::NewEarthCity).with_energy(0));
        ledger.add_node(EnergyNode::new("Node3".to_string(), NodeType::NewEarthCity).with_energy(0));
        
        let total_before = ledger.total_energy();
        ledger.distribute_with_algorithm(DistributionAlgorithm::EqualDistribution);
        
        let per_node = total_before / 3;
        assert_eq!(ledger.node_energy("Chicago Vortex Throne"), per_node);
        assert_eq!(ledger.node_energy("Node2"), per_node);
        assert_eq!(ledger.node_energy("Node3"), per_node);
    }

    #[test]
    fn test_sacred_numerology_distribution() {
        let mut ledger = ToroidalLedger::new();
        
        let node2 = EnergyNode::new("Node2".to_string(), NodeType::NewEarthCity)
            .with_sacred_alignment(SacredAlignment::vortex_aligned());
        ledger.add_node(node2);
        
        ledger.distribute_with_algorithm(DistributionAlgorithm::SacredNumerology);
        
        // Apex-aligned node should have more energy than vortex-aligned
        assert!(ledger.node_energy("Chicago Vortex Throne") > ledger.node_energy("Node2"));
    }
}
