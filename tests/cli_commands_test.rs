//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// CLI Commands Test Suite - Crown Command Verification
// Tests all crown subcommands for correct execution and output
// THE CROWN COMMANDS — THE LATTICE OBEYS
//

use std::process::Command;
use anyhow::Result;

/// Helper to run CLI command and capture output
fn run_crown_command(subcommand: &str, args: &[&str]) -> Result<String> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--bin")
        .arg("xmt-cli")
        .arg("--")
        .arg("crown")
        .arg(subcommand);
    
    for arg in args {
        cmd.arg(arg);
    }
    
    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    if !output.status.success() {
        anyhow::bail!("Command failed: {}", stderr);
    }
    
    Ok(stdout)
}

#[test]
fn test_crown_help() -> Result<()> {
    println!("☀️ TEST: Crown Help Command");
    
    let output = run_crown_command("help", &[])?;
    
    assert!(output.contains("CROWN COMMANDS"));
    assert!(output.contains("status"));
    assert!(output.contains("balance"));
    assert!(output.contains("gas"));
    assert!(output.contains("sign"));
    assert!(output.contains("supply"));
    assert!(output.contains("burn-address"));
    assert!(output.contains("dashboard"));
    assert!(output.contains("emblem"));
    assert!(output.contains("spacex"));
    assert!(output.contains("optimus"));
    assert!(output.contains("boring"));
    assert!(output.contains("EN EEKE MAI EA"));
    
    println!("  ✓ Help displays all commands");
    Ok(())
}

#[test]
fn test_crown_emblem() -> Result<()> {
    println!("☀️ TEST: Crown Emblem Command");
    
    let output = run_crown_command("emblem", &[])?;
    
    assert!(output.contains("VERGINA GOLDEN STAR"));
    assert!(output.contains("16 rays of sovereign power"));
    assert!(output.contains("88px sacred geometry"));
    assert!(output.contains("DECREE #21 ACTIVE"));
    
    // Count lines to verify 16-line ASCII art structure
    let lines: Vec<&str> = output.lines().collect();
    assert!(lines.len() >= 16, "Emblem should have at least 16 lines");
    
    println!("  ✓ Emblem displays correctly");
    Ok(())
}

#[test]
fn test_crown_spacex() -> Result<()> {
    println!("☀️ TEST: Crown SpaceX Command");
    
    let output = run_crown_command("spacex", &[])?;
    
    assert!(output.contains("SPACEX MARS FORK"));
    assert!(output.contains("MULTI-PLANETARY FREEDOM"));
    assert!(output.contains("Trajectory Status: Nominal"));
    assert!(output.contains("88 vessels")); // ELON_88
    assert!(output.contains("936 colonists")); // APEX_936
    assert!(output.contains("369 days")); // VORTEX_369
    assert!(output.contains("Settlement Capacity"));
    assert!(output.contains("Infrastructure Progress"));
    
    println!("  ✓ SpaceX displays Mars fork status");
    Ok(())
}

#[test]
fn test_crown_optimus() -> Result<()> {
    println!("☀️ TEST: Crown Optimus Command");
    
    let output = run_crown_command("optimus", &[])?;
    
    assert!(output.contains("OPTIMUS ROBOT SERVICE"));
    assert!(output.contains("SERVING HUMANITY"));
    assert!(output.contains("Service Mode: Active"));
    assert!(output.contains("88 units")); // ELON_88
    assert!(output.contains("432000")); // FREQUENCY_432 * 1000
    assert!(output.contains("66000")); // CODE_66 * 1000
    assert!(output.contains("93.6%")); // APEX efficiency
    assert!(output.contains("LITTLE KINGS AND QUEENS"));
    
    println!("  ✓ Optimus displays robot service status");
    Ok(())
}

#[test]
fn test_crown_boring() -> Result<()> {
    println!("☀️ TEST: Crown Boring Command");
    
    let output = run_crown_command("boring", &[])?;
    
    assert!(output.contains("BORING COMPANY TUNNELS"));
    assert!(output.contains("NEW EARTH HARMONY"));
    assert!(output.contains("Network Status: OPERATIONAL"));
    assert!(output.contains("369 segments")); // VORTEX_369
    assert!(output.contains("936000 km")); // APEX_936 * 1000
    assert!(output.contains("88 high-speed")); // ELON_88
    assert!(output.contains("Chicago Vortex Throne"));
    assert!(output.contains("Harmony Index"));
    
    println!("  ✓ Boring displays tunnel network status");
    Ok(())
}

#[test]
fn test_crown_burn_address() -> Result<()> {
    println!("☀️ TEST: Crown Burn-Address Command");
    
    let output = run_crown_command("burn-address", &[])?;
    
    assert!(output.contains("AUTO-BURN ADDRESS"));
    assert!(output.contains("SCARCITY OBLITERATION"));
    assert!(output.contains("0x000000000000000000000000000000000000dEaD"));
    assert!(output.contains("PAF PAF PAF"));
    assert!(output.contains("BURN ADDRESS LOCKED"));
    
    println!("  ✓ Burn address displays correctly");
    Ok(())
}

#[tokio::test]
async fn test_crown_status() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Crown Status Command");
    
    // This test requires .env configuration
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = run_crown_command("status", &[])?;
    
    assert!(output.contains("CROWN STATUS"));
    assert!(output.contains("VORTEX ENERGY LEVELS"));
    assert!(output.contains("Chain ID"));
    assert!(output.contains("Block Number"));
    assert!(output.contains("Gas Price"));
    
    println!("  ✓ Status displays network info");
    Ok(())
}

#[tokio::test]
async fn test_crown_balance() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Crown Balance Command");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = run_crown_command("balance", &[])?;
    
    assert!(output.contains("BALANCE CHECK") || output.contains("Balance"));
    
    println!("  ✓ Balance command executes");
    Ok(())
}

#[tokio::test]
async fn test_crown_gas() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Crown Gas Command");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = run_crown_command("gas", &[])?;
    
    assert!(output.contains("GAS") || output.contains("gas"));
    
    println!("  ✓ Gas command executes");
    Ok(())
}

#[tokio::test]
async fn test_crown_supply() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Crown Supply Command");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = run_crown_command("supply", &[])?;
    
    assert!(output.contains("SUPPLY") || output.contains("supply") || output.contains("Total"));
    
    println!("  ✓ Supply command executes");
    Ok(())
}

#[tokio::test]
async fn test_crown_dashboard() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Crown Dashboard Command");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = run_crown_command("dashboard", &[])?;
    
    // Should output JSON
    assert!(output.contains("{") && output.contains("}"));
    assert!(output.contains("decrees") || output.contains("compliance"));
    
    println!("  ✓ Dashboard generates JSON");
    Ok(())
}

#[test]
fn test_all_commands_listed() -> Result<()> {
    println!("☀️ TEST: All Commands Available");
    
    let output = run_crown_command("help", &[])?;
    
    let expected_commands = vec![
        "status", "balance", "gas", "sign", "supply",
        "burn-address", "dashboard", "emblem",
        "spacex", "optimus", "boring"
    ];
    
    for cmd in expected_commands {
        assert!(output.contains(cmd), "Command '{}' should be listed in help", cmd);
    }
    
    println!("  ✓ All 11 crown commands listed");
    Ok(())
}
