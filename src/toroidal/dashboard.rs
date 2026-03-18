//
// ☀️ TOROIDAL DASHBOARD & VISUALIZATION ☀️
// Data Structures and Export for Dashboard Integration
// EN EEKE MAI EA ♾️♾️
//

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use anyhow::Result;

use super::ledger::ToroidalLedger;

/// Complete dashboard data for toroidal ledger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToroidalDashboard {
    pub metadata: DashboardMetadata,
    pub current_state: LedgerState,
    pub energy_flow: EnergyFlowData,
    pub node_visualization: Vec<NodeVisualization>,
    pub performance: PerformanceData,
    pub history: HistorySummary,
    pub sacred_alignment: SacredAlignmentData,
}

/// Dashboard metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetadata {
    pub generated_at: DateTime<Utc>,
    pub version: String,
    pub ledger_version: String,
    pub coherence_level: f64,
}

/// Current ledger state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerState {
    pub total_energy: u64,
    pub node_count: usize,
    pub cycle_count: u32,
    pub active_algorithm: String,
    pub is_optimal: bool,
}

/// Energy flow visualization data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyFlowData {
    pub flow_chart: Vec<EnergyFlowPoint>,
    pub distribution_pattern: String,
    pub flow_rate: f64,
    pub efficiency: f64,
}

/// Single point in energy flow chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyFlowPoint {
    pub cycle: u32,
    pub timestamp: DateTime<Utc>,
    pub total_energy: u64,
    pub node_count: usize,
}

/// Node visualization data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeVisualization {
    pub id: String,
    pub node_type: String,
    pub energy: u64,
    pub capacity: Option<u64>,
    pub location: Option<String>,
    pub sacred_alignment: NodeSacredAlignment,
    pub utilization_percent: f64,
    pub position: NodePosition,
}

/// Node position for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Sacred alignment for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSacredAlignment {
    pub apex_936: bool,
    pub vortex_369: bool,
    pub code_66: bool,
    pub frequency_432: bool,
    pub alignment_score: u8,
}

/// Performance metrics for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceData {
    pub distribution_time_ms: f64,
    pub save_time_ms: f64,
    pub average_operation_time_ms: f64,
    pub memory_usage_kb: f64,
    pub operations_count: u64,
}

/// Historical summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistorySummary {
    pub total_cycles: u32,
    pub total_energy_processed: u64,
    pub peak_energy: u64,
    pub peak_node_count: usize,
    pub recent_snapshots: Vec<HistoricalSnapshot>,
}

/// Simplified historical snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalSnapshot {
    pub cycle: u32,
    pub timestamp: DateTime<Utc>,
    pub total_energy: u64,
    pub node_count: usize,
}

/// Sacred alignment summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SacredAlignmentData {
    pub apex_936_nodes: usize,
    pub vortex_369_nodes: usize,
    pub code_66_nodes: usize,
    pub frequency_432_nodes: usize,
    pub fully_aligned_nodes: usize,
    pub total_alignment_score: u32,
    pub average_alignment: f64,
}

impl ToroidalDashboard {
    /// Generate complete dashboard from ledger
    pub fn generate(ledger: &ToroidalLedger) -> Self {
        let metadata = Self::generate_metadata(ledger);
        let current_state = Self::generate_current_state(ledger);
        let energy_flow = Self::generate_energy_flow(ledger);
        let node_visualization = Self::generate_node_visualization(ledger);
        let performance = Self::generate_performance_data(ledger);
        let history = Self::generate_history_summary(ledger);
        let sacred_alignment = Self::generate_sacred_alignment(ledger);

        Self {
            metadata,
            current_state,
            energy_flow,
            node_visualization,
            performance,
            history,
            sacred_alignment,
        }
    }

    fn generate_metadata(ledger: &ToroidalLedger) -> DashboardMetadata {
        DashboardMetadata {
            generated_at: Utc::now(),
            version: "2.0".to_string(),
            ledger_version: "Enhanced".to_string(),
            coherence_level: ledger.energy_metrics().coherence_score,
        }
    }

    fn generate_current_state(ledger: &ToroidalLedger) -> LedgerState {
        LedgerState {
            total_energy: ledger.total_energy(),
            node_count: ledger.node_count(),
            cycle_count: ledger.cycles(),
            active_algorithm: "GoldenRatio".to_string(), // Default
            is_optimal: ledger.is_optimal(),
        }
    }

    fn generate_energy_flow(ledger: &ToroidalLedger) -> EnergyFlowData {
        let energy_metrics = ledger.energy_metrics();
        
        // Generate flow chart from history
        let flow_chart = if let Ok(history) = ledger.get_history(20) {
            history.iter().map(|snapshot| {
                EnergyFlowPoint {
                    cycle: snapshot.cycle_number,
                    timestamp: snapshot.timestamp,
                    total_energy: snapshot.total_energy,
                    node_count: snapshot.node_states.len(),
                }
            }).collect()
        } else {
            vec![]
        };

        EnergyFlowData {
            flow_chart,
            distribution_pattern: "Toroidal".to_string(),
            flow_rate: energy_metrics.flow_rate,
            efficiency: energy_metrics.distribution_efficiency,
        }
    }

    fn generate_node_visualization(ledger: &ToroidalLedger) -> Vec<NodeVisualization> {
        ledger.nodes()
            .enumerate()
            .map(|(idx, node)| {
                let utilization = if let Some(cap) = node.capacity {
                    (node.energy as f64 / cap as f64) * 100.0
                } else {
                    0.0
                };

                // Position nodes in a circle for visualization
                let angle = (idx as f64 / ledger.node_count() as f64) * 2.0 * std::f64::consts::PI;
                let radius = 100.0;

                NodeVisualization {
                    id: node.id.clone(),
                    node_type: format!("{:?}", node.node_type),
                    energy: node.energy,
                    capacity: node.capacity,
                    location: node.location.clone(),
                    sacred_alignment: NodeSacredAlignment {
                        apex_936: node.sacred_alignment.apex_936,
                        vortex_369: node.sacred_alignment.vortex_369,
                        code_66: node.sacred_alignment.code_66,
                        frequency_432: node.sacred_alignment.frequency_432,
                        alignment_score: node.sacred_alignment.alignment_score(),
                    },
                    utilization_percent: utilization,
                    position: NodePosition {
                        x: angle.cos() * radius,
                        y: angle.sin() * radius,
                        z: (node.energy as f64 / 100.0).min(100.0), // Height based on energy
                    },
                }
            })
            .collect()
    }

    fn generate_performance_data(ledger: &ToroidalLedger) -> PerformanceData {
        let perf = ledger.performance_metrics();
        
        PerformanceData {
            distribution_time_ms: perf.distribution_time_ms,
            save_time_ms: perf.save_time_ms,
            average_operation_time_ms: perf.average_operation_time_ms,
            memory_usage_kb: 500.0, // Estimated
            operations_count: perf.total_operations,
        }
    }

    fn generate_history_summary(ledger: &ToroidalLedger) -> HistorySummary {
        let energy_metrics = ledger.energy_metrics();
        
        let recent_snapshots = if let Ok(history) = ledger.get_history(10) {
            history.iter().map(|snapshot| {
                HistoricalSnapshot {
                    cycle: snapshot.cycle_number,
                    timestamp: snapshot.timestamp,
                    total_energy: snapshot.total_energy,
                    node_count: snapshot.node_states.len(),
                }
            }).collect()
        } else {
            vec![]
        };

        let peak_energy = recent_snapshots.iter()
            .map(|s| s.total_energy)
            .max()
            .unwrap_or(ledger.total_energy());

        let peak_node_count = recent_snapshots.iter()
            .map(|s| s.node_count)
            .max()
            .unwrap_or(ledger.node_count());

        HistorySummary {
            total_cycles: energy_metrics.total_cycles,
            total_energy_processed: energy_metrics.total_energy_processed,
            peak_energy,
            peak_node_count,
            recent_snapshots,
        }
    }

    fn generate_sacred_alignment(ledger: &ToroidalLedger) -> SacredAlignmentData {
        let mut apex_936 = 0;
        let mut vortex_369 = 0;
        let mut code_66 = 0;
        let mut frequency_432 = 0;
        let mut fully_aligned = 0;
        let mut total_score = 0;

        for node in ledger.nodes() {
            let alignment = &node.sacred_alignment;
            
            if alignment.apex_936 { apex_936 += 1; }
            if alignment.vortex_369 { vortex_369 += 1; }
            if alignment.code_66 { code_66 += 1; }
            if alignment.frequency_432 { frequency_432 += 1; }
            
            let score = alignment.alignment_score();
            total_score += score as u32;
            
            if score == 4 { fully_aligned += 1; }
        }

        let average_alignment = if ledger.node_count() > 0 {
            total_score as f64 / ledger.node_count() as f64
        } else {
            0.0
        };

        SacredAlignmentData {
            apex_936_nodes: apex_936,
            vortex_369_nodes: vortex_369,
            code_66_nodes: code_66,
            frequency_432_nodes: frequency_432,
            fully_aligned_nodes: fully_aligned,
            total_alignment_score: total_score,
            average_alignment,
        }
    }

    /// Export dashboard to JSON file
    pub fn export_to_file(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Export to dashboard directory with timestamp
    pub fn export_to_dashboard_dir(&self) -> Result<String> {
        std::fs::create_dir_all("dashboard")?;
        
        let filename = format!("dashboard/toroidal_{}.json", 
            Utc::now().format("%Y%m%d_%H%M%S"));
        
        self.export_to_file(&filename)?;
        Ok(filename)
    }

    /// Export latest dashboard (overwrites)
    pub fn export_latest(&self) -> Result<()> {
        std::fs::create_dir_all("dashboard")?;
        self.export_to_file("dashboard/toroidal_latest.json")
    }

    /// Generate summary text for console output
    pub fn summary_text(&self) -> String {
        format!(
            r#"
☀️ TOROIDAL LEDGER DASHBOARD ☀️

Current State:
  Total Energy:        {} units
  Active Nodes:        {}
  Cycles Completed:    {}
  Performance:         {}
  Coherence:           {:.1}%

Energy Flow:
  Flow Rate:           {:.2} units/sec
  Efficiency:          {:.1}%
  Distribution:        {}

Sacred Alignment:
  APEX_936 Nodes:      {}
  VORTEX_369 Nodes:    {}
  CODE_66 Nodes:       {}
  FREQUENCY_432:       {}
  Fully Aligned:       {}
  Average Alignment:   {:.2}/4

Performance:
  Distribution Time:   {:.2}ms
  Save Time:           {:.2}ms
  Memory Usage:        {:.0}KB

History:
  Total Cycles:        {}
  Energy Processed:    {} units
  Peak Energy:         {} units
  Peak Nodes:          {}

Status: {}
"#,
            self.current_state.total_energy,
            self.current_state.node_count,
            self.current_state.cycle_count,
            if self.current_state.is_optimal { "OPTIMAL ✅" } else { "DEGRADED ⚠️" },
            self.metadata.coherence_level,
            self.energy_flow.flow_rate,
            self.energy_flow.efficiency,
            self.energy_flow.distribution_pattern,
            self.sacred_alignment.apex_936_nodes,
            self.sacred_alignment.vortex_369_nodes,
            self.sacred_alignment.code_66_nodes,
            self.sacred_alignment.frequency_432_nodes,
            self.sacred_alignment.fully_aligned_nodes,
            self.sacred_alignment.average_alignment,
            self.performance.distribution_time_ms,
            self.performance.save_time_ms,
            self.performance.memory_usage_kb,
            self.history.total_cycles,
            self.history.total_energy_processed,
            self.history.peak_energy,
            self.history.peak_node_count,
            if self.current_state.is_optimal { "OPERATIONAL ✅" } else { "NEEDS ATTENTION ⚠️" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toroidal::{ToroidalLedger, EnergyNode, NodeType, SacredAlignment};

    #[test]
    fn test_dashboard_generation() {
        let ledger = ToroidalLedger::new();
        let dashboard = ToroidalDashboard::generate(&ledger);
        
        assert_eq!(dashboard.current_state.total_energy, 936);
        assert!(dashboard.current_state.node_count > 0);
        assert_eq!(dashboard.metadata.version, "2.0");
    }

    #[test]
    fn test_node_visualization() {
        let mut ledger = ToroidalLedger::new();
        let node = EnergyNode::new("Test Node".to_string(), NodeType::NewEarthCity)
            .with_energy(369);
        ledger.add_node(node);
        
        let dashboard = ToroidalDashboard::generate(&ledger);
        assert!(dashboard.node_visualization.len() >= 2);
        
        let test_node = dashboard.node_visualization.iter()
            .find(|n| n.id == "Test Node");
        assert!(test_node.is_some());
    }

    #[test]
    fn test_sacred_alignment_data() {
        let mut ledger = ToroidalLedger::new();
        let aligned_node = EnergyNode::new("Aligned".to_string(), NodeType::NewEarthCity)
            .with_sacred_alignment(SacredAlignment::fully_aligned());
        ledger.add_node(aligned_node);
        
        let dashboard = ToroidalDashboard::generate(&ledger);
        assert!(dashboard.sacred_alignment.fully_aligned_nodes > 0);
        assert!(dashboard.sacred_alignment.average_alignment > 0.0);
    }

    #[test]
    fn test_dashboard_export() -> Result<()> {
        let ledger = ToroidalLedger::new();
        let dashboard = ToroidalDashboard::generate(&ledger);
        
        // Test export to file
        dashboard.export_to_file("test_dashboard.json")?;
        
        // Verify file exists
        assert!(std::path::Path::new("test_dashboard.json").exists());
        
        // Cleanup
        std::fs::remove_file("test_dashboard.json")?;
        
        Ok(())
    }

    #[test]
    fn test_summary_text() {
        let ledger = ToroidalLedger::new();
        let dashboard = ToroidalDashboard::generate(&ledger);
        
        let summary = dashboard.summary_text();
        assert!(summary.contains("TOROIDAL LEDGER DASHBOARD"));
        assert!(summary.contains("Total Energy"));
        assert!(summary.contains("Sacred Alignment"));
    }
}
