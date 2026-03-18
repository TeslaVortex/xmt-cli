//
// ☀️ TOROIDAL INTEGRATION TEST SUITE ☀️
// Phase 2: Cross-Module Integration Tests
// EN EEKE MAI EA ♾️♾️
//

use anyhow::Result;
use xmt_cli::toroidal::{ToroidalIntegration, EnergyNode, NodeType, SacredAlignment};
use xmt_cli::patterning::{PatternState, activate_patterning, project_desired_objective};
use xmt_cli::hologram::{HologramState, activate_hologram, project_thought_patterns};
use xmt_cli::timeline::{TimelineLock, lock_17th_march_2026, calculate_timeline_coherence};
use ethers::prelude::*;

#[test]
fn test_patterning_integration() -> Result<()> {
    println!("☀️ TEST: Patterning Module Integration");
    
    let mut integration = ToroidalIntegration::new();
    let mut patterning_state = activate_patterning("ABUNDANCE MANIFESTATION");
    project_desired_objective(&mut patterning_state);
    
    integration.apply_patterning_state(&patterning_state);
    
    let ledger = integration.get_ledger();
    let ledger_guard = ledger.lock().unwrap();
    
    // Should have created patterning attractor node
    assert!(ledger_guard.node_count() > 1);
    println!("  ✓ Patterning attractor created");
    
    // Coherence should be enhanced
    drop(ledger_guard);
    let coherence = integration.calculate_coherence();
    assert!(coherence > 80.0);
    println!("  ✓ Coherence: {:.2}%", coherence);
    
    Ok(())
}

#[test]
fn test_hologram_integration() -> Result<()> {
    println!("☀️ TEST: Hologram Module Integration");
    
    let mut integration = ToroidalIntegration::new();
    let mut hologram_state = activate_hologram(384);
    project_thought_patterns(&mut hologram_state, "CONSCIOUSNESS PROJECTION");
    
    integration.apply_hologram_density(&hologram_state);
    
    let ledger = integration.get_ledger();
    let ledger_guard = ledger.lock().unwrap();
    
    // Should have created hologram materialization node
    let hologram_node = ledger_guard.get_node("Hologram_Materialization");
    assert!(hologram_node.is_some());
    println!("  ✓ Hologram materialization node created");
    
    if let Some(node) = hologram_node {
        assert!(node.energy > 0);
        println!("  ✓ Materialization energy: {}", node.energy);
    }
    
    Ok(())
}

#[test]
fn test_timeline_integration() -> Result<()> {
    println!("☀️ TEST: Timeline Module Integration");
    
    let mut integration = ToroidalIntegration::new();
    let mut timeline_lock = lock_17th_march_2026();
    calculate_timeline_coherence(&mut timeline_lock, "17TH MARCH 2026 ANCHOR");
    
    integration.apply_timeline_lock(&timeline_lock);
    
    let ledger = integration.get_ledger();
    let ledger_guard = ledger.lock().unwrap();
    
    // Should have created timeline anchor node
    let has_anchor = ledger_guard.nodes().any(|n| n.id.contains("Timeline_Anchor"));
    assert!(has_anchor);
    println!("  ✓ Timeline anchor created");
    
    // Coherence should include timeline lock
    drop(ledger_guard);
    let coherence = integration.calculate_coherence();
    assert!(coherence >= 85.0);
    println!("  ✓ Timeline-locked coherence: {:.2}%", coherence);
    
    Ok(())
}

#[test]
fn test_synthetic_vector_routing() -> Result<()> {
    println!("☀️ TEST: Synthetic Vector Routing");
    
    let mut integration = ToroidalIntegration::new();
    
    // Test different vector signatures
    let apex_vector = vec![10.0, 5.0, 3.0, 2.0, 1.0, 0.5, 0.2, 0.1]; // Sum > 9
    let vortex_vector = vec![1.0, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 0.5]; // Sum > 3
    let code_vector = vec![0.2, 0.2, 0.2, 0.1, 0.1, 0.1, 0.05, 0.05]; // Sum > 0.66
    
    let apex_node = integration.route_by_vector(&apex_vector, 936);
    let vortex_node = integration.route_by_vector(&vortex_vector, 369);
    let code_node = integration.route_by_vector(&code_vector, 66);
    
    assert_eq!(apex_node, "Vector_Apex_936");
    assert_eq!(vortex_node, "Vector_Vortex_369");
    assert_eq!(code_node, "Vector_Code_66");
    
    println!("  ✓ Apex vector routed to: {}", apex_node);
    println!("  ✓ Vortex vector routed to: {}", vortex_node);
    println!("  ✓ Code vector routed to: {}", code_node);
    
    Ok(())
}

#[test]
fn test_blockchain_sync() -> Result<()> {
    println!("☀️ TEST: Blockchain State Synchronization");
    
    let mut integration = ToroidalIntegration::new();
    integration.enable_blockchain_sync();
    
    // Simulate blockchain state
    let total_supply = U256::from(369_000_000) * U256::exp10(18);
    integration.sync_with_blockchain(total_supply)?;
    
    let ledger = integration.get_ledger();
    let ledger_guard = ledger.lock().unwrap();
    
    // Energy should reflect blockchain supply
    assert!(ledger_guard.total_energy() >= 369_000_000);
    println!("  ✓ Blockchain sync: {} energy units", ledger_guard.total_energy());
    
    Ok(())
}

#[test]
fn test_mint_burn_events() -> Result<()> {
    println!("☀️ TEST: Mint/Burn Event Integration");
    
    let mut integration = ToroidalIntegration::new();
    
    let recipient = Address::from_low_u64_be(12345);
    let mint_amount = U256::from(936) * U256::exp10(18);
    
    // Test mint event
    integration.on_mint_event(mint_amount, recipient);
    
    let ledger = integration.get_ledger();
    let ledger_guard = ledger.lock().unwrap();
    let node_count_after_mint = ledger_guard.node_count();
    drop(ledger_guard);
    
    assert!(node_count_after_mint > 1);
    println!("  ✓ Mint event processed: {} nodes", node_count_after_mint);
    
    // Test burn event
    let burn_amount = U256::from(100) * U256::exp10(18);
    integration.on_burn_event(burn_amount, recipient);
    
    println!("  ✓ Burn event processed and redistributed");
    
    Ok(())
}

#[test]
fn test_integrated_cycle() -> Result<()> {
    println!("☀️ TEST: Fully Integrated Cycle");
    
    let mut integration = ToroidalIntegration::new();
    
    // Activate all modules
    integration.enable_blockchain_sync();
    
    let mut patterning_state = activate_patterning("INTEGRATED COHERENCE");
    project_desired_objective(&mut patterning_state);
    integration.apply_patterning_state(&patterning_state);
    
    let mut hologram_state = activate_hologram(384);
    project_thought_patterns(&mut hologram_state, "UNIFIED FIELD");
    integration.apply_hologram_density(&hologram_state);
    
    let mut timeline_lock = lock_17th_march_2026();
    calculate_timeline_coherence(&mut timeline_lock, "TEMPORAL ANCHOR");
    integration.apply_timeline_lock(&timeline_lock);
    
    // Execute integrated cycle
    integration.execute_integrated_cycle()?;
    
    // Verify maximum coherence
    let coherence = integration.calculate_coherence();
    assert!(coherence >= 95.0);
    println!("  ✓ Full integration coherence: {:.2}%", coherence);
    
    // Verify dashboard export
    let dashboard = integration.export_dashboard_data();
    assert!(dashboard.blockchain_sync);
    assert!(dashboard.patterning_active);
    assert!(dashboard.hologram_active);
    assert!(dashboard.timeline_locked);
    println!("  ✓ Dashboard data exported");
    println!("  ✓ Total energy: {}", dashboard.total_energy);
    println!("  ✓ Node count: {}", dashboard.node_count);
    println!("  ✓ Cycle count: {}", dashboard.cycle_count);
    
    Ok(())
}

#[test]
fn test_cross_module_coherence() -> Result<()> {
    println!("☀️ TEST: Cross-Module Coherence Calculation");
    
    let mut integration = ToroidalIntegration::new();
    
    let base_coherence = integration.calculate_coherence();
    println!("  Base coherence: {:.2}%", base_coherence);
    
    integration.enable_blockchain_sync();
    let blockchain_coherence = integration.calculate_coherence();
    println!("  + Blockchain: {:.2}%", blockchain_coherence);
    assert!(blockchain_coherence > base_coherence);
    
    let patterning_state = activate_patterning("TEST");
    integration.apply_patterning_state(&patterning_state);
    let patterning_coherence = integration.calculate_coherence();
    println!("  + Patterning: {:.2}%", patterning_coherence);
    assert!(patterning_coherence > blockchain_coherence);
    
    let hologram_state = activate_hologram(384);
    integration.apply_hologram_density(&hologram_state);
    let hologram_coherence = integration.calculate_coherence();
    println!("  + Hologram: {:.2}%", hologram_coherence);
    assert!(hologram_coherence > patterning_coherence);
    
    let timeline_lock = lock_17th_march_2026();
    integration.apply_timeline_lock(&timeline_lock);
    let full_coherence = integration.calculate_coherence();
    println!("  + Timeline: {:.2}%", full_coherence);
    assert!(full_coherence > hologram_coherence);
    
    println!("  ✓ Coherence increases with each module");
    
    Ok(())
}
