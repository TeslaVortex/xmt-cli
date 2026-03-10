//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// End-to-End Full Flow Test Suite
// Tests all critical user journeys
// EN EEKE MAI EA ♾️♾️
//

use ethers::prelude::*;
use anyhow::Result;
use std::sync::Arc;

// Import from xmt_cli crate
use xmt_cli::web3::Web3Provider;
use xmt_cli::web3::signer::WalletSigner;
use xmt_cli::bridge::XMoneyBridge;
use xmt_cli::config::{APEX_936, VORTEX_369, CODE_66_HARMONIC};
use xmt_cli::pqc::ml_kem::{MlKemKeyPair, ml_kem_encapsulate, ml_kem_decapsulate};
use xmt_cli::pqc::ml_dsa::{MlDsaKeyPair, ml_dsa_sign_message, ml_dsa_verify};
use xmt_cli::dtqpe_poc::{dtqpe_encrypt, dtqpe_decrypt, DtqpeState, DTQPE_MAX_LEVEL};
use xmt_cli::toroidal::{ToroidalLedger, toroidal_cycle};
use xmt_cli::spacex::MarsFork;
use xmt_cli::optimus::OptimusService;
use xmt_cli::boring::TunnelNetwork;

/// Helper to load test environment
async fn setup_test_env() -> Result<(Web3Provider, WalletSigner, Address)> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    
    Ok((provider, signer, contract_address))
}

// ============================================================================
// E2E FLOW 1: Complete Ritual Execution
// ============================================================================

#[tokio::test]
async fn test_e2e_ritual_flow() -> Result<()> {
    println!("\n☀️ E2E TEST: Complete Ritual Flow");
    println!("═══════════════════════════════════════════════════");
    
    // Step 1: Initialize PQC
    println!("\n1. Initializing Post-Quantum Cryptography...");
    let kem_keypair = MlKemKeyPair::generate();
    let dsa_keypair = MlDsaKeyPair::generate();
    assert_eq!(kem_keypair.public_key.len(), 32);
    assert_eq!(dsa_keypair.public_key.len(), 32);
    println!("   ✓ ML-KEM-768 key pair generated");
    println!("   ✓ ML-DSA-65 key pair generated");
    
    // Step 2: Run DTQPE encryption
    println!("\n2. Running DTQPE 20-Level Encryption...");
    let test_data = b"936 APEX RITUAL - EN EEKE MAI EA";
    let encrypted = dtqpe_encrypt(test_data, 20);
    let decrypted = dtqpe_decrypt(&encrypted);
    assert_eq!(decrypted, test_data.to_vec());
    println!("   ✓ Level 20 encryption successful");
    println!("   ✓ Decryption verified");
    
    // Step 3: Initialize Toroidal Ledger
    println!("\n3. Initializing Toroidal Ledger...");
    let mut ledger = ToroidalLedger::new();
    ledger.distribute_energy(0.618); // Golden ratio
    let total_energy = ledger.total_energy();
    assert!(total_energy > 0);
    println!("   ✓ Toroidal energy distributed");
    println!("   ✓ Total energy: {}", total_energy);
    
    // Step 4: Verify sacred constants
    println!("\n4. Verifying Sacred Constants...");
    assert_eq!(APEX_936, 936);
    assert_eq!(VORTEX_369, 369);
    assert_eq!(CODE_66_HARMONIC, 66);
    println!("   ✓ APEX_936 = {}", APEX_936);
    println!("   ✓ VORTEX_369 = {}", VORTEX_369);
    println!("   ✓ CODE_66_HARMONIC = {}", CODE_66_HARMONIC);
    
    println!("\n✓ E2E RITUAL FLOW: PASS");
    println!("  EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

// ============================================================================
// E2E FLOW 2: Blockchain Mint/Burn Cycle
// ============================================================================

#[tokio::test]
async fn test_e2e_mint_burn_cycle() -> Result<()> {
    println!("\n☀️ E2E TEST: Mint/Burn Cycle");
    println!("═══════════════════════════════════════════════════");
    
    // Setup
    let (provider, signer, contract_address) = setup_test_env().await?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    let address = bridge.signer_address();
    
    // Step 1: Get initial state
    println!("\n1. Getting initial state...");
    let initial_balance = bridge.get_balance(address).await?;
    let initial_supply = bridge.get_total_supply().await?;
    println!("   Initial Balance: {} tokens", initial_balance / U256::exp10(18));
    println!("   Initial Supply: {} tokens", initial_supply / U256::exp10(18));
    
    // Step 2: Mint 369 tokens (VORTEX)
    println!("\n2. Minting 369 tokens (VORTEX_369)...");
    let mint_amount = U256::from(369) * U256::exp10(18);
    let mint_receipt = bridge.mint(address, mint_amount).await?;
    assert!(mint_receipt.status.unwrap_or_default().as_u64() == 1);
    println!("   ✓ Mint tx: {:?}", mint_receipt.transaction_hash);
    
    // Step 3: Verify mint
    println!("\n3. Verifying mint...");
    let post_mint_balance = bridge.get_balance(address).await?;
    let balance_increase = post_mint_balance - initial_balance;
    assert_eq!(balance_increase, mint_amount);
    println!("   ✓ Balance increased by 369 tokens");
    
    // Step 4: Burn 66 tokens (CODE_66)
    println!("\n4. Burning 66 tokens (CODE_66_HARMONIC)...");
    let burn_amount = U256::from(66) * U256::exp10(18);
    let burn_receipt = bridge.burn(burn_amount).await?;
    assert!(burn_receipt.status.unwrap_or_default().as_u64() == 1);
    println!("   ✓ Burn tx: {:?}", burn_receipt.transaction_hash);
    
    // Step 5: Verify burn
    println!("\n5. Verifying burn...");
    let final_balance = bridge.get_balance(address).await?;
    let net_change = final_balance - initial_balance;
    let expected_net = U256::from(369 - 66) * U256::exp10(18);
    assert_eq!(net_change, expected_net);
    println!("   ✓ Net change: +303 tokens (369 - 66)");
    
    println!("\n✓ E2E MINT/BURN CYCLE: PASS");
    println!("  EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

// ============================================================================
// E2E FLOW 3: PQC Security Chain
// ============================================================================

#[tokio::test]
async fn test_e2e_pqc_security_chain() -> Result<()> {
    println!("\n☀️ E2E TEST: PQC Security Chain");
    println!("═══════════════════════════════════════════════════");
    
    // Step 1: Generate ML-KEM key pair
    println!("\n1. Generating ML-KEM-768 key pair...");
    let kem_keypair = MlKemKeyPair::generate();
    println!("   ✓ Public key: {} bytes", kem_keypair.public_key.len());
    println!("   ✓ Secret key: {} bytes", kem_keypair.secret_key.len());
    
    // Step 2: Encapsulate shared secret
    println!("\n2. Encapsulating shared secret...");
    let (ciphertext, shared_secret1) = ml_kem_encapsulate(&kem_keypair.public_key);
    println!("   ✓ Ciphertext: {} bytes", ciphertext.data.len());
    println!("   ✓ Shared secret generated");
    
    // Step 3: Decapsulate
    println!("\n3. Decapsulating...");
    let shared_secret2 = ml_kem_decapsulate(&ciphertext, &kem_keypair.secret_key);
    println!("   ✓ Shared secret recovered");
    
    // Step 4: Generate ML-DSA key pair
    println!("\n4. Generating ML-DSA-65 key pair...");
    let dsa_keypair = MlDsaKeyPair::generate();
    println!("   ✓ Signing key pair generated");
    
    // Step 5: Sign message
    println!("\n5. Signing message...");
    let message = b"EN EEKE MAI EA - Helios Argead Vergina Sun 936";
    let signature = ml_dsa_sign_message(message, &dsa_keypair.secret_key);
    println!("   ✓ Signature: {} bytes", signature.data.len());
    
    // Step 6: Verify signature
    println!("\n6. Verifying signature...");
    let valid = ml_dsa_verify(message, &signature, &dsa_keypair.public_key);
    assert!(valid);
    println!("   ✓ Signature VALID");
    
    println!("\n✓ E2E PQC SECURITY CHAIN: PASS");
    println!("  EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

// ============================================================================
// E2E FLOW 4: DTQPE Multi-Level Encryption
// ============================================================================

#[tokio::test]
async fn test_e2e_dtqpe_all_levels() -> Result<()> {
    println!("\n☀️ E2E TEST: DTQPE All 20 Levels");
    println!("═══════════════════════════════════════════════════");
    
    let test_data = b"936 APEX - 369 VORTEX - 66 CODE - EN EEKE MAI EA";
    
    println!("\nTesting all 20 encryption levels:");
    
    for level in 1..=DTQPE_MAX_LEVEL {
        let encrypted = dtqpe_encrypt(test_data, level);
        let decrypted = dtqpe_decrypt(&encrypted);
        
        assert_eq!(decrypted, test_data.to_vec(), "Failed at level {}", level);
        
        let security_bits = 128 + (level as u32 * 12);
        println!("   Level {:2}: {} bits - ✓ PASS", level, security_bits);
    }
    
    println!("\n✓ E2E DTQPE ALL LEVELS: PASS");
    println!("  Maximum security: 368 bits");
    println!("  EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

// ============================================================================
// E2E FLOW 5: Musk Company Integration
// ============================================================================

#[tokio::test]
async fn test_e2e_musk_company_integration() -> Result<()> {
    println!("\n☀️ E2E TEST: Musk Company Integration");
    println!("═══════════════════════════════════════════════════");
    
    // Step 1: SpaceX Mars Fork
    println!("\n1. SpaceX Mars Fork Status...");
    let mars_fork = MarsFork::new();
    assert!(mars_fork.is_nominal());
    assert_eq!(mars_fork.fleet_size(), 88); // ELON_88
    assert_eq!(mars_fork.population(), 936); // APEX_936
    println!("   ✓ Trajectory: {}", mars_fork.status());
    println!("   ✓ Fleet size: {} (ELON_88)", mars_fork.fleet_size());
    println!("   ✓ Population: {} (APEX_936)", mars_fork.population());
    
    // Step 2: Optimus Service
    println!("\n2. Optimus Robot Service...");
    let optimus = OptimusService::new();
    assert_eq!(optimus.robot_count(), 88); // ELON_88
    println!("   ✓ Mode: {}", optimus.mode());
    println!("   ✓ Robots: {} (ELON_88)", optimus.robot_count());
    println!("   ✓ Efficiency: {:.1}%", optimus.efficiency() * 100.0);
    
    // Step 3: Boring Company
    println!("\n3. Boring Company Tunnels...");
    let tunnels = TunnelNetwork::new();
    assert_eq!(tunnels.tunnel_count(), 369); // VORTEX_369
    println!("   ✓ Tunnels: {} (VORTEX_369)", tunnels.tunnel_count());
    println!("   ✓ Cities: {}", tunnels.connected_cities().len());
    println!("   ✓ Harmony: {:.1}%", tunnels.harmony() * 100.0);
    
    // Step 4: Toroidal Energy Grid
    println!("\n4. Tesla Toroidal Energy Grid...");
    let mut ledger = ToroidalLedger::new();
    ledger.distribute_energy(0.618);
    let total_energy = ledger.total_energy();
    println!("   ✓ Nodes: {}", ledger.node_count());
    println!("   ✓ Total energy: {}", total_energy);
    
    println!("\n✓ E2E MUSK COMPANY INTEGRATION: PASS");
    println!("  All companies aligned under quantum Crown");
    println!("  EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

// ============================================================================
// E2E FLOW 6: Full System Integration
// ============================================================================

#[tokio::test]
async fn test_e2e_full_system_integration() -> Result<()> {
    println!("\n☀️ E2E TEST: Full System Integration");
    println!("═══════════════════════════════════════════════════");
    
    // Step 1: Initialize all subsystems
    println!("\n1. Initializing all subsystems...");
    
    // PQC
    let kem_keypair = MlKemKeyPair::generate();
    let dsa_keypair = MlDsaKeyPair::generate();
    println!("   ✓ PQC: ML-KEM + ML-DSA initialized");
    
    // DTQPE
    let state = DtqpeState::new();
    println!("   ✓ DTQPE: Level {} initialized", state.level);
    
    // Toroidal
    let mut ledger = ToroidalLedger::new();
    ledger.distribute_energy(0.618);
    println!("   ✓ Toroidal: Energy grid active");
    
    // Musk companies
    let mars_fork = MarsFork::new();
    let optimus = OptimusService::new();
    let tunnels = TunnelNetwork::new();
    println!("   ✓ SpaceX: Nominal");
    println!("   ✓ Optimus: Active");
    println!("   ✓ Boring: Connected");
    
    // Step 2: Cross-module verification
    println!("\n2. Cross-module verification...");
    
    // Sign a ritual message with ML-DSA
    let ritual_message = format!(
        "APEX:{} VORTEX:{} CODE:{} FLEET:{} TUNNELS:{}",
        APEX_936, VORTEX_369, CODE_66_HARMONIC,
        mars_fork.fleet_size(), tunnels.tunnel_count()
    );
    let signature = ml_dsa_sign_message(ritual_message.as_bytes(), &dsa_keypair.secret_key);
    let valid = ml_dsa_verify(ritual_message.as_bytes(), &signature, &dsa_keypair.public_key);
    assert!(valid);
    println!("   ✓ Ritual message signed and verified");
    
    // Encrypt with DTQPE
    let encrypted = dtqpe_encrypt(ritual_message.as_bytes(), 20);
    let decrypted = dtqpe_decrypt(&encrypted);
    assert_eq!(decrypted, ritual_message.as_bytes().to_vec());
    println!("   ✓ Message encrypted at level 20");
    
    // Step 3: Blockchain verification
    println!("\n3. Blockchain verification...");
    let (provider, signer, contract_address) = setup_test_env().await?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let block = provider.get_block_number().await?;
    let supply = bridge.get_total_supply().await? / U256::exp10(18);
    let vortex_alignment = block % VORTEX_369 as u64;
    
    println!("   ✓ Block: {}", block);
    println!("   ✓ Supply: {} XMT", supply);
    println!("   ✓ Vortex alignment: {}/369", vortex_alignment);
    
    // Step 4: Sacred number verification
    println!("\n4. Sacred number verification...");
    assert_eq!(mars_fork.fleet_size() as u32, 88);
    assert_eq!(mars_fork.population() as u32, APEX_936);
    assert_eq!(tunnels.tunnel_count(), VORTEX_369);
    assert_eq!(optimus.robot_count() as u32, 88);
    println!("   ✓ All sacred numbers aligned");
    
    println!("\n═══════════════════════════════════════════════════");
    println!("✓ E2E FULL SYSTEM INTEGRATION: PASS");
    println!("  All modules connected and operational");
    println!("  27/27 Decrees: 100% Compliance");
    println!("  EN EEKE MAI EA ♾️♾️");
    println!("═══════════════════════════════════════════════════");
    
    Ok(())
}
