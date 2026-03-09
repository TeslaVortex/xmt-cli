//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Decree Modules Test Suite - SpaceX, Optimus, Boring, Toroidal
// Tests all decree module functionality and sacred number calculations
// THE CROWN COMMANDS — THE LATTICE OBEYS
//

use anyhow::Result;
use xmt_cli::spacex::{MarsFork, mars_fork_nominal};
use xmt_cli::optimus::{OptimusService, optimus_ready};
use xmt_cli::boring::{TunnelNetwork, tunnel_ready};
use xmt_cli::toroidal::ToroidalLedger;
use xmt_cli::config::{APEX_936, VORTEX_369, ELON_88, CODE_66_HARMONIC, FREQUENCY_432};

#[test]
fn test_spacex_mars_fork_creation() -> Result<()> {
    println!("☀️ TEST: SpaceX Mars Fork Creation");
    
    let fork = MarsFork::new();
    
    assert_eq!(fork.fleet_size(), 88, "Fleet should be ELON_88");
    assert_eq!(fork.population(), 936, "Population should be APEX_936");
    assert_eq!(fork.launch_cadence(), 369, "Cadence should be VORTEX_369");
    assert!(fork.is_nominal(), "Trajectory should be nominal");
    assert_eq!(fork.status(), "Nominal");
    
    println!("  ✓ Mars fork initialized with sacred numbers");
    Ok(())
}

#[test]
fn test_spacex_settlement_capacity() -> Result<()> {
    println!("☀️ TEST: SpaceX Settlement Capacity");
    
    let fork = MarsFork::new();
    let capacity = fork.settlement_capacity();
    
    // 936 * (936 / 100) = 936 * 9.36 = 8760.96 ≈ 8760
    assert!(capacity > 8000, "Settlement capacity should be > 8000");
    assert!(capacity < 10000, "Settlement capacity should be < 10000");
    
    println!("  ✓ Settlement capacity: {}", capacity);
    Ok(())
}

#[test]
fn test_spacex_launch_window() -> Result<()> {
    println!("☀️ TEST: SpaceX Launch Window Calculation");
    
    let fork = MarsFork::new();
    let window = fork.next_launch_window();
    
    // Should be (369 % 369) + 1 = 1
    assert_eq!(window, 1, "Next launch window should be 1 day");
    
    println!("  ✓ Next launch window: {} days", window);
    Ok(())
}

#[test]
fn test_spacex_nominal_status() -> Result<()> {
    println!("☀️ TEST: SpaceX Nominal Status Function");
    
    assert!(mars_fork_nominal(), "Mars fork should always be nominal");
    
    println!("  ✓ Mars fork trajectory nominal");
    Ok(())
}

#[test]
fn test_optimus_service_creation() -> Result<()> {
    println!("☀️ TEST: Optimus Service Creation");
    
    let service = OptimusService::new();
    
    assert_eq!(service.robot_count(), 88, "Fleet should be ELON_88");
    assert_eq!(service.tasks_completed(), 432000, "Tasks should be FREQUENCY_432 * 1000");
    assert_eq!(service.people_served(), 66000, "Served should be CODE_66 * 1000");
    assert_eq!(service.mode(), "Active");
    
    println!("  ✓ Optimus service initialized with sacred numbers");
    Ok(())
}

#[test]
fn test_optimus_efficiency() -> Result<()> {
    println!("☀️ TEST: Optimus Service Efficiency");
    
    let service = OptimusService::new();
    let efficiency = service.efficiency();
    
    assert_eq!(efficiency, 0.936, "Efficiency should be 93.6% (APEX)");
    
    println!("  ✓ Service efficiency: {:.1}%", efficiency * 100.0);
    Ok(())
}

#[test]
fn test_optimus_harmonic_ratio() -> Result<()> {
    println!("☀️ TEST: Optimus Harmonic Ratio");
    
    let service = OptimusService::new();
    let ratio = service.harmonic_ratio();
    
    // 432000 / 66 = 6545.45... capped at 1000
    assert_eq!(ratio, 1000.0, "Harmonic ratio should be capped at 1000");
    
    println!("  ✓ Harmonic ratio: {:.1}", ratio);
    Ok(())
}

#[test]
fn test_optimus_frequency_alignment() -> Result<()> {
    println!("☀️ TEST: Optimus Frequency Alignment");
    
    let service = OptimusService::new();
    let alignment = service.frequency_alignment();
    
    // 432000 / 432 = 1000
    assert_eq!(alignment, 1000, "Frequency alignment should be 1000 cycles");
    
    println!("  ✓ Frequency alignment: {} cycles", alignment);
    Ok(())
}

#[test]
fn test_optimus_ready_status() -> Result<()> {
    println!("☀️ TEST: Optimus Ready Status Function");
    
    assert!(optimus_ready(), "Optimus service should be ready");
    
    println!("  ✓ Optimus service operational");
    Ok(())
}

#[test]
fn test_boring_tunnel_network_creation() -> Result<()> {
    println!("☀️ TEST: Boring Tunnel Network Creation");
    
    let network = TunnelNetwork::new();
    
    assert_eq!(network.tunnel_count(), 369, "Tunnels should be VORTEX_369");
    assert_eq!(network.total_length(), 936000, "Length should be APEX_936 * 1000");
    assert_eq!(network.hyperloop_count(), 88, "Hyperloop should be ELON_88");
    assert_eq!(network.connected_cities().len(), 6, "Should have 6 cities");
    
    println!("  ✓ Tunnel network initialized with sacred numbers");
    Ok(())
}

#[test]
fn test_boring_connected_cities() -> Result<()> {
    println!("☀️ TEST: Boring Connected Cities");
    
    let network = TunnelNetwork::new();
    let cities = network.connected_cities();
    
    assert!(cities.contains(&"Chicago Vortex Throne".to_string()));
    assert!(cities.contains(&"New Atlantis".to_string()));
    assert!(cities.contains(&"Vergina Star Hub".to_string()));
    assert!(cities.contains(&"Mars Fork Alpha".to_string()));
    assert!(cities.contains(&"Tesla Grid Central".to_string()));
    assert!(cities.contains(&"Quantum Crown City".to_string()));
    
    println!("  ✓ All 6 cities connected");
    Ok(())
}

#[test]
fn test_boring_harmony_index() -> Result<()> {
    println!("☀️ TEST: Boring Harmony Index");
    
    let network = TunnelNetwork::new();
    let harmony = network.harmony();
    
    assert_eq!(harmony, 0.936, "Harmony should be 93.6% (APEX)");
    
    println!("  ✓ Harmony index: {:.1}%", harmony * 100.0);
    Ok(())
}

#[test]
fn test_boring_vortex_connectivity() -> Result<()> {
    println!("☀️ TEST: Boring Vortex Connectivity");
    
    let network = TunnelNetwork::new();
    let connectivity = network.vortex_connectivity();
    
    // 369 / 369 * 100 = 100%
    assert_eq!(connectivity, 100.0, "Vortex connectivity should be 100%");
    
    println!("  ✓ Vortex connectivity: {:.1}%", connectivity);
    Ok(())
}

#[test]
fn test_boring_infrastructure_completion() -> Result<()> {
    println!("☀️ TEST: Boring Infrastructure Completion");
    
    let network = TunnelNetwork::new();
    let completion = network.infrastructure_completion();
    
    // 936000 / (936 * 1000) * 100 = 100%
    assert_eq!(completion, 100.0, "Infrastructure should be 100% complete");
    
    println!("  ✓ Infrastructure completion: {:.1}%", completion);
    Ok(())
}

#[test]
fn test_boring_ready_status() -> Result<()> {
    println!("☀️ TEST: Boring Ready Status Function");
    
    assert!(tunnel_ready(), "Tunnel infrastructure should be ready");
    
    println!("  ✓ Tunnel network operational");
    Ok(())
}

#[test]
fn test_toroidal_ledger_creation() -> Result<()> {
    println!("☀️ TEST: Toroidal Ledger Creation");
    
    let ledger = ToroidalLedger::new();
    
    // Ledger starts with 936 energy in Chicago Vortex Throne
    assert_eq!(ledger.total_energy(), 936, "Initial energy should be 936 (APEX)");
    assert_eq!(ledger.node_count(), 1, "Should have 1 initial node");
    
    println!("  ✓ Toroidal ledger initialized");
    Ok(())
}

#[test]
fn test_toroidal_energy_distribution() -> Result<()> {
    println!("☀️ TEST: Toroidal Energy Distribution");
    
    let mut ledger = ToroidalLedger::new();
    
    // Add more nodes
    ledger.add_node("Node 2");
    ledger.add_node("Node 3");
    ledger.add_node("Node 4");
    
    // Distribute with golden ratio (0.618)
    ledger.distribute_energy(0.618);
    
    let total = ledger.total_energy();
    assert!(total > 0, "Total energy should be positive");
    
    println!("  ✓ Energy distributed: {} total", total);
    Ok(())
}

#[test]
fn test_toroidal_golden_ratio_distribution() -> Result<()> {
    println!("☀️ TEST: Toroidal Golden Ratio Distribution");
    
    let mut ledger = ToroidalLedger::new();
    
    // Add energy to multiple nodes
    ledger.add_energy("Vortex Node", 369); // VORTEX
    ledger.add_energy("Frequency Node", 432); // FREQUENCY
    ledger.add_energy("Harmonic Node", 66);  // CODE_66
    ledger.add_energy("Infinite Node", 88);  // ELON_88
    
    let initial_total = ledger.total_energy();
    
    // Distribute with golden ratio
    ledger.distribute_energy(0.618);
    
    let total = ledger.total_energy();
    assert!(total >= initial_total, "Total energy should not decrease");
    
    println!("  ✓ Golden ratio distribution complete");
    Ok(())
}

#[test]
fn test_all_modules_sacred_numbers() -> Result<()> {
    println!("☀️ TEST: All Modules Sacred Number Alignment");
    
    let fork = MarsFork::new();
    let service = OptimusService::new();
    let network = TunnelNetwork::new();
    
    // Verify ELON_88 appears in all modules
    assert_eq!(fork.fleet_size(), ELON_88 as u32);
    assert_eq!(service.robot_count(), ELON_88 as u32);
    assert_eq!(network.hyperloop_count(), ELON_88 as u32);
    
    // Verify APEX_936
    assert_eq!(fork.population(), APEX_936 as u64);
    assert_eq!(network.total_length(), (APEX_936 as u64) * 1000);
    
    // Verify VORTEX_369
    assert_eq!(fork.launch_cadence(), VORTEX_369 as u32);
    assert_eq!(network.tunnel_count(), VORTEX_369 as u32);
    
    // Verify FREQUENCY_432
    assert_eq!(service.tasks_completed(), (FREQUENCY_432 as u64) * 1000);
    
    // Verify CODE_66_HARMONIC
    assert_eq!(service.people_served(), (CODE_66_HARMONIC as u64) * 1000);
    
    println!("  ✓ All sacred numbers aligned across modules");
    Ok(())
}
