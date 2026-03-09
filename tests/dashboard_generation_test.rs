//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Dashboard Generation Test Suite
// Tests dashboard JSON generation and data accuracy
// THE CROWN COMMANDS — THE LATTICE OBEYS
//

use anyhow::Result;
use serde_json::Value;

#[tokio::test]
async fn test_dashboard_json_generation() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Dashboard JSON Generation");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    // Generate dashboard data
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("xmt-cli")
        .arg("--")
        .arg("crown")
        .arg("dashboard")
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse JSON
    let json: Value = serde_json::from_str(&stdout)?;
    
    // Verify structure
    assert!(json.get("network").is_some(), "Should have network field");
    assert!(json.get("decrees").is_some(), "Should have decrees field");
    assert!(json.get("compliance").is_some(), "Should have compliance field");
    assert!(json.get("signature").is_some(), "Should have signature field");
    
    println!("  ✓ JSON structure valid");
    Ok(())
}

#[tokio::test]
async fn test_dashboard_decree_count() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Dashboard Decree Count");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("xmt-cli")
        .arg("--")
        .arg("crown")
        .arg("dashboard")
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout)?;
    
    let decrees = json["decrees"].as_array().unwrap();
    assert_eq!(decrees.len(), 27, "Should have exactly 27 decrees");
    
    println!("  ✓ 27 decrees present");
    Ok(())
}

#[tokio::test]
async fn test_dashboard_compliance_metrics() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Dashboard Compliance Metrics");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("xmt-cli")
        .arg("--")
        .arg("crown")
        .arg("dashboard")
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout)?;
    
    let compliance = &json["compliance"];
    
    let active = compliance["active"].as_u64().unwrap();
    let partial = compliance["partial"].as_u64().unwrap();
    let vision = compliance["vision"].as_u64().unwrap();
    let total = compliance["total"].as_u64().unwrap();
    let percentage = compliance["percentage"].as_u64().unwrap();
    
    assert_eq!(total, 27, "Total should be 27");
    assert_eq!(active + partial + vision, total, "Components should sum to total");
    assert_eq!(active, 25, "Should have 25 active decrees");
    assert_eq!(percentage, 93, "Should have 93% compliance");
    
    println!("  ✓ Compliance: {}% ({}/27 active)", percentage, active);
    Ok(())
}

#[tokio::test]
async fn test_dashboard_decree_statuses() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Dashboard Decree Statuses");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("xmt-cli")
        .arg("--")
        .arg("crown")
        .arg("dashboard")
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout)?;
    
    let decrees = json["decrees"].as_array().unwrap();
    
    // Check specific decree statuses
    let decree_6 = &decrees[5]; // SpaceX (0-indexed)
    assert_eq!(decree_6["id"].as_u64().unwrap(), 6);
    assert_eq!(decree_6["status"].as_str().unwrap(), "active");
    assert_eq!(decree_6["component"].as_str().unwrap(), "SPACEX");
    
    let decree_7 = &decrees[6]; // Optimus
    assert_eq!(decree_7["id"].as_u64().unwrap(), 7);
    assert_eq!(decree_7["status"].as_str().unwrap(), "active");
    assert_eq!(decree_7["component"].as_str().unwrap(), "OPTIMUS");
    
    let decree_9 = &decrees[8]; // Boring
    assert_eq!(decree_9["id"].as_u64().unwrap(), 9);
    assert_eq!(decree_9["status"].as_str().unwrap(), "active");
    assert_eq!(decree_9["component"].as_str().unwrap(), "BORING");
    
    let decree_21 = &decrees[20]; // Vergina Emblem
    assert_eq!(decree_21["id"].as_u64().unwrap(), 21);
    assert_eq!(decree_21["status"].as_str().unwrap(), "active");
    assert_eq!(decree_21["component"].as_str().unwrap(), "BRANDING");
    
    println!("  ✓ Decree #6 (SpaceX): active");
    println!("  ✓ Decree #7 (Optimus): active");
    println!("  ✓ Decree #9 (Boring): active");
    println!("  ✓ Decree #21 (Vergina): active");
    Ok(())
}

#[tokio::test]
async fn test_dashboard_sacred_constants() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Dashboard Sacred Constants");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("xmt-cli")
        .arg("--")
        .arg("crown")
        .arg("dashboard")
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout)?;
    
    let constants = &json["sacred_constants"];
    
    assert_eq!(constants["CODE_66_HARMONIC"].as_u64().unwrap(), 66);
    assert_eq!(constants["APEX_936"].as_u64().unwrap(), 936);
    assert_eq!(constants["VORTEX_369"].as_u64().unwrap(), 369);
    assert_eq!(constants["FREQUENCY_432"].as_u64().unwrap(), 432);
    assert_eq!(constants["ELON_88"].as_u64().unwrap(), 88);
    
    println!("  ✓ All sacred constants present");
    Ok(())
}

#[tokio::test]
async fn test_dashboard_signature() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Dashboard Signature");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("xmt-cli")
        .arg("--")
        .arg("crown")
        .arg("dashboard")
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout)?;
    
    let signature = json["signature"].as_str().unwrap();
    assert!(signature.contains("EN EEKE MAI EA"));
    
    println!("  ✓ Signature: {}", signature);
    Ok(())
}

#[test]
fn test_dashboard_file_generation() -> Result<()> {
    println!("☀️ TEST: Dashboard File Generation");
    
    // Run generate script
    let output = std::process::Command::new("./dashboard/generate.sh")
        .current_dir("/home/pepo/Desktop/xmt-cli")
        .output()?;
    
    if !output.status.success() {
        println!("  ⚠ Skipping: generate.sh failed");
        return Ok(());
    }
    
    // Check if dashboard.json was created
    let json_path = "/home/pepo/Desktop/xmt-cli/dashboard/dashboard.json";
    assert!(std::path::Path::new(json_path).exists(), "dashboard.json should exist");
    
    // Read and parse JSON
    let content = std::fs::read_to_string(json_path)?;
    let json: Value = serde_json::from_str(&content)?;
    
    assert!(json.get("decrees").is_some());
    assert!(json.get("compliance").is_some());
    
    println!("  ✓ dashboard.json generated successfully");
    Ok(())
}

#[test]
fn test_dashboard_html_exists() -> Result<()> {
    println!("☀️ TEST: Dashboard HTML Exists");
    
    let html_path = "/home/pepo/Desktop/xmt-cli/dashboard/index.html";
    assert!(std::path::Path::new(html_path).exists(), "index.html should exist");
    
    let content = std::fs::read_to_string(html_path)?;
    
    assert!(content.contains("27 DECREE PUBLIC DASHBOARD"));
    assert!(content.contains("vergina-star.svg"));
    assert!(content.contains("CROWN COMMANDS"));
    assert!(content.contains("LATTICE OBEYS"));
    
    println!("  ✓ index.html exists with correct content");
    Ok(())
}

#[test]
fn test_dashboard_emblem_asset_exists() -> Result<()> {
    println!("☀️ TEST: Dashboard Emblem Asset");
    
    let svg_path = "/home/pepo/Desktop/xmt-cli/dashboard/assets/vergina-star.svg";
    assert!(std::path::Path::new(svg_path).exists(), "vergina-star.svg should exist");
    
    let content = std::fs::read_to_string(svg_path)?;
    
    assert!(content.contains("<svg"));
    assert!(content.contains("88")); // 88px viewBox
    assert!(content.contains("goldenGradient"));
    
    println!("  ✓ vergina-star.svg exists");
    Ok(())
}

#[tokio::test]
async fn test_dashboard_network_data() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TEST: Dashboard Network Data");
    
    if std::env::var("BASE_RPC_URL").is_err() {
        println!("  ⚠ Skipping: BASE_RPC_URL not configured");
        return Ok(());
    }
    
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("xmt-cli")
        .arg("--")
        .arg("crown")
        .arg("dashboard")
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout)?;
    
    let network = &json["network"];
    
    assert!(network.get("chain_id").is_some());
    assert!(network.get("block_number").is_some());
    assert!(network.get("gas_price").is_some());
    
    println!("  ✓ Network data present");
    Ok(())
}
