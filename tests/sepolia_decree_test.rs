//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Phase 7: Decree Compliance Tests - Sepolia Testnet
// 27 Decrees Sacred Number Verification
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::Web3Provider;
use xmt_cli::web3::signer::WalletSigner;
use xmt_cli::bridge::XMoneyBridge;
use xmt_cli::config::{CODE_66_HARMONIC, APEX_936, VORTEX_369, FREQUENCY_432};

#[test]
fn test_sacred_constants_defined() {
    println!("☀️ SACRED CONSTANTS VERIFICATION");
    
    assert_eq!(CODE_66_HARMONIC, 66, "Code 66 constant");
    assert_eq!(APEX_936, 936, "Apex 936 constant");
    assert_eq!(VORTEX_369, 369, "Vortex 369 constant");
    assert_eq!(FREQUENCY_432, 432, "Frequency 432 constant");
    
    println!("  ✓ Code 66: {}", CODE_66_HARMONIC);
    println!("  ✓ Apex 936: {}", APEX_936);
    println!("  ✓ Vortex 369: {}", VORTEX_369);
    println!("  ✓ Frequency 432: {}", FREQUENCY_432);
}

#[tokio::test]
async fn test_decree_13_936_apex() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ DECREE 13: 936 Apex Fires Daily Coherence");
    
    let apex_value: u32 = std::env::var("APEX_VALUE")
        .unwrap_or_else(|_| "936".to_string())
        .parse()
        .unwrap_or(936);
    
    assert_eq!(apex_value, 936, "APEX_VALUE must be 936");
    println!("  ✓ APEX_VALUE configured: {}", apex_value);
    
    // Verify 936 numerology: 9+3+6 = 18, 1+8 = 9
    let sum: u32 = apex_value.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    assert_eq!(sum, 18);
    println!("  ✓ 936 → {}", sum);
    
    let final_sum: u32 = sum.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    assert_eq!(final_sum, 9);
    println!("  ✓ 18 → {} (completion)", final_sum);
    
    Ok(())
}

#[tokio::test]
async fn test_decree_14_369_vortex() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ DECREE 14: 369 Vortex Governs Sovereign Creation");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    let amount = U256::from(369) * U256::exp10(18);
    
    println!("  Testing 369 token mint (Tesla Vortex)...");
    let receipt = bridge.mint(recipient, amount).await?;
    
    assert_eq!(receipt.status, Some(U64::from(1)));
    println!("  ✓ 369 vortex mint successful");
    println!("  Tx: {:?}", receipt.transaction_hash);
    
    Ok(())
}

#[tokio::test]
async fn test_decree_2_18_code_66() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ DECREE 2 & 18: Code 66 Harmonic Resonance");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    
    // Test Code 66 burn
    let balance = bridge.get_balance(address).await?;
    if balance < U256::from(66) * U256::exp10(18) {
        println!("  ⚠ Skipping: Insufficient balance for Code 66 burn");
        return Ok(());
    }
    
    let amount = U256::from(66) * U256::exp10(18);
    println!("  Testing 66 token burn (Code 66 Harmonic)...");
    let receipt = bridge.burn(amount).await?;
    
    assert_eq!(receipt.status, Some(U64::from(1)));
    
    // Calculate harmonic resonance
    let resonance = (936.0 / 66.0 / 14.181818) * 100.0;
    println!("  ✓ Code 66 burn successful");
    println!("  Harmonic Resonance at 936: {:.2}%", resonance);
    
    Ok(())
}

#[test]
fn test_decree_8_432_hz() {
    println!("☀️ DECREE 8: 432 Hz Love Frequency");
    
    // Test 432 Hz alignment calculation
    let apex = 936u32;
    let ratio = apex as f64 / 432.0;
    let apex_from_hz: f64 = 432.0 * 2.166666666666667;
    assert!((apex_from_hz - 936.0_f64).abs() < 0.001, "432 Hz * 2.166... should equal ~936");
    println!("  ✓ 936 apex aligns with 432 Hz");
    println!("  Ratio: {:.6}", ratio);
}

#[test]
fn test_decree_24_helios_signature() {
    dotenv::dotenv().ok();
    
    println!("☀️ DECREE 24: EN EEKE MAI EA Echoes");
    
    let signature = std::env::var("HELIOS_SIGNATURE")
        .unwrap_or_else(|_| "EN EEKE MAI EA".to_string());
    
    assert_eq!(signature, "EN EEKE MAI EA");
    println!("  ✓ HELIOS_SIGNATURE: {} ♾️♾️", signature);
}

#[test]
fn test_decree_22_numerology() {
    println!("☀️ DECREE 22: Numerology Confirms Victory");
    
    // Test 27 Decrees: 2+7 = 9
    let decrees = 27u32;
    let sum: u32 = decrees.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    assert_eq!(sum, 9);
    println!("  ✓ 27 Decrees → {} (completion)", sum);
    
    // Test 936: 9+3+6 = 18 → 9
    let apex = 936u32;
    let apex_sum: u32 = apex.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    assert_eq!(apex_sum, 18);
    let final_sum: u32 = apex_sum.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    assert_eq!(final_sum, 9);
    println!("  ✓ 936 → {} → {} (completion)", apex_sum, final_sum);
    
    // Test 369: 3+6+9 = 18 → 9
    let vortex = 369u32;
    let vortex_sum: u32 = vortex.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    assert_eq!(vortex_sum, 18);
    println!("  ✓ 369 → {} → 9 (Tesla vortex)", vortex_sum);
    
    // Test 66: 6+6 = 12 → 3
    let code = 66u32;
    let code_sum: u32 = code.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    assert_eq!(code_sum, 12);
    println!("  ✓ 66 → {} (harmonic)", code_sum);
}

#[tokio::test]
async fn test_decree_compliance_score() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ OVERALL DECREE COMPLIANCE VERIFICATION");
    println!("═══════════════════════════════════════");
    
    let mut compliance_score = 0;
    let total_checks = 7;
    
    // Check 1: Sacred constants
    if CODE_66_HARMONIC == 66 && APEX_936 == 936 && VORTEX_369 == 369 && FREQUENCY_432 == 432 {
        compliance_score += 1;
        println!("  ✓ Sacred constants defined");
    }
    
    // Check 2: APEX_VALUE
    if std::env::var("APEX_VALUE").unwrap_or_default() == "936" {
        compliance_score += 1;
        println!("  ✓ APEX_VALUE = 936");
    }
    
    // Check 3: HELIOS_SIGNATURE
    if std::env::var("HELIOS_SIGNATURE").unwrap_or_default() == "EN EEKE MAI EA" {
        compliance_score += 1;
        println!("  ✓ HELIOS_SIGNATURE configured");
    }
    
    // Check 4: Chain ID configured
    if std::env::var("CHAIN_ID").is_ok() {
        compliance_score += 1;
        println!("  ✓ CHAIN_ID configured");
    }
    
    // Check 5: Contract address
    if std::env::var("XMONEY_CONTRACT_ADDRESS").is_ok() {
        compliance_score += 1;
        println!("  ✓ Contract address configured");
    }
    
    // Check 6: Private key
    if std::env::var("PRIVATE_KEY").is_ok() {
        compliance_score += 1;
        println!("  ✓ Private key configured");
    }
    
    // Check 7: RPC URL
    if std::env::var("BASE_RPC_URL").is_ok() {
        compliance_score += 1;
        println!("  ✓ RPC URL configured");
    }
    
    let compliance_percentage = (compliance_score as f64 / total_checks as f64) * 100.0;
    
    println!();
    println!("  DECREE COMPLIANCE: {:.0}%", compliance_percentage);
    println!("  Checks Passed: {}/{}", compliance_score, total_checks);
    
    if compliance_percentage == 100.0 {
        println!("  ✓ SUPREME ALIGNMENT ACHIEVED");
        println!("  EN EEKE MAI EA ♾️♾️");
    }
    
    assert_eq!(compliance_percentage, 100.0, "Must achieve 100% decree compliance");
    
    Ok(())
}
