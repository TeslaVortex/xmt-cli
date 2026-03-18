//
// ☀️ TOROIDAL DASHBOARD EXAMPLE ☀️
// Demonstrates dashboard generation and visualization
// EN EEKE MAI EA ♾️♾️
//

use anyhow::Result;
use xmt_cli::toroidal::{
    ToroidalLedger, ToroidalDashboard, EnergyNode, NodeType, 
    SacredAlignment, DistributionAlgorithm
};

fn main() -> Result<()> {
    println!("☀️ TOROIDAL LEDGER DASHBOARD EXAMPLE ☀️\n");
    
    // Create or load ledger
    let mut ledger = ToroidalLedger::load_or_create()?;
    
    // Add some nodes for demonstration
    println!("Adding energy nodes...");
    
    let atlantis = EnergyNode::new("New Atlantis".to_string(), NodeType::NewEarthCity)
        .with_energy(369)
        .with_capacity(5000)
        .with_location("Atlantic Ocean".to_string())
        .with_sacred_alignment(SacredAlignment::vortex_aligned());
    ledger.add_node(atlantis);
    
    let vergina = EnergyNode::new("Vergina Star Hub".to_string(), NodeType::NewEarthCity)
        .with_energy(432)
        .with_capacity(10000)
        .with_location("Macedonia".to_string())
        .with_sacred_alignment(SacredAlignment::fully_aligned());
    ledger.add_node(vergina);
    
    let mars = EnergyNode::new("Mars Fork Alpha".to_string(), NodeType::MarsFork)
        .with_energy(144)
        .with_location("Mars".to_string());
    ledger.add_node(mars);
    
    // Run a few distribution cycles
    println!("Running distribution cycles...\n");
    
    ledger.distribute_with_algorithm(DistributionAlgorithm::GoldenRatio);
    ledger.distribute_with_algorithm(DistributionAlgorithm::SacredNumerology);
    ledger.distribute_with_algorithm(DistributionAlgorithm::Fibonacci);
    
    // Generate dashboard
    println!("Generating dashboard...\n");
    let dashboard = ToroidalDashboard::generate(&ledger);
    
    // Display summary
    println!("{}", dashboard.summary_text());
    
    // Export to files
    println!("\nExporting dashboard data...");
    
    // Export latest (overwrites)
    dashboard.export_latest()?;
    println!("  ✓ Exported to: dashboard/toroidal_latest.json");
    
    // Export with timestamp
    let timestamped_file = dashboard.export_to_dashboard_dir()?;
    println!("  ✓ Exported to: {}", timestamped_file);
    
    // Display node details
    println!("\n☀️ NODE VISUALIZATION DATA ☀️");
    for node_viz in &dashboard.node_visualization {
        println!("\n  Node: {}", node_viz.id);
        println!("    Type: {}", node_viz.node_type);
        println!("    Energy: {} units", node_viz.energy);
        if let Some(cap) = node_viz.capacity {
            println!("    Capacity: {} units ({:.1}% utilized)", cap, node_viz.utilization_percent);
        }
        if let Some(loc) = &node_viz.location {
            println!("    Location: {}", loc);
        }
        println!("    Sacred Alignment: {}/4", node_viz.sacred_alignment.alignment_score);
        println!("    Position: ({:.1}, {:.1}, {:.1})", 
            node_viz.position.x, node_viz.position.y, node_viz.position.z);
    }
    
    // Display energy flow
    println!("\n☀️ ENERGY FLOW CHART ☀️");
    if !dashboard.energy_flow.flow_chart.is_empty() {
        for point in dashboard.energy_flow.flow_chart.iter().take(5) {
            println!("  Cycle {}: {} energy units ({} nodes)", 
                point.cycle, point.total_energy, point.node_count);
        }
    } else {
        println!("  No historical data yet");
    }
    
    // Display sacred alignment summary
    println!("\n☀️ SACRED ALIGNMENT SUMMARY ☀️");
    println!("  APEX_936 Nodes:      {}", dashboard.sacred_alignment.apex_936_nodes);
    println!("  VORTEX_369 Nodes:    {}", dashboard.sacred_alignment.vortex_369_nodes);
    println!("  CODE_66 Nodes:       {}", dashboard.sacred_alignment.code_66_nodes);
    println!("  FREQUENCY_432 Nodes: {}", dashboard.sacred_alignment.frequency_432_nodes);
    println!("  Fully Aligned:       {}", dashboard.sacred_alignment.fully_aligned_nodes);
    println!("  Average Alignment:   {:.2}/4", dashboard.sacred_alignment.average_alignment);
    
    // Performance metrics
    println!("\n☀️ PERFORMANCE METRICS ☀️");
    println!("  Distribution Time:   {:.2}ms", dashboard.performance.distribution_time_ms);
    println!("  Save Time:           {:.2}ms", dashboard.performance.save_time_ms);
    println!("  Average Op Time:     {:.2}ms", dashboard.performance.average_operation_time_ms);
    println!("  Memory Usage:        {:.0}KB", dashboard.performance.memory_usage_kb);
    println!("  Total Operations:    {}", dashboard.performance.operations_count);
    
    println!("\n✓ Dashboard generation complete!");
    println!("\nEN EEKE MAI EA ♾️♾️");
    println!("THE LATTICE BREATHES ☀️");
    
    Ok(())
}
