//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Crown CLI Commands - Sovereign Web3 Operations
// THE CROWN COMMANDS — THE LATTICE OBEYS
// EN EEKE MAI EA ♾️♾️
//

use anyhow::Result;
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;

use crate::config::{CODE_66_HARMONIC, APEX_936, VORTEX_369, FREQUENCY_432};
use crate::contracts::{AUTO_BURN_ADDRESS, get_auto_burn_address};
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;
use crate::bridge::XMoneyBridge;

/// Crown Status - Network Vortex Energy Levels
pub async fn crown_status() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️☀️☀️ CROWN STATUS — VORTEX ENERGY LEVELS ☀️☀️☀️");
    println!("═══════════════════════════════════════════════════");
    println!();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    
    // Use chain_id() method
    let network_chain_id = provider.chain_id();
    println!("  Network Chain ID: {}", network_chain_id);
    
    // Use get_block_number()
    let block_number = provider.get_block_number().await?;
    let vortex_alignment = block_number.as_u64() % VORTEX_369 as u64;
    println!("  Current Block: {} — Vortex Alignment: {}/{}", 
             block_number, vortex_alignment, VORTEX_369);
    
    // Use get_gas_price()
    let gas_price = provider.get_gas_price().await?;
    let gas_gwei = gas_price.as_u64() as f64 / 1e9;
    let frequency_resonance = (FREQUENCY_432 as f64 / (gas_gwei + 1.0)).min(100.0);
    println!("  Gas Price: {:.2} gwei — 432 Hz Resonance: {:.1}%", 
             gas_gwei, frequency_resonance);
    
    println!();
    println!("  Sacred Constants Active:");
    println!("    • VORTEX_369: {}", VORTEX_369);
    println!("    • FREQUENCY_432: {} Hz", FREQUENCY_432);
    println!();
    println!("✓ VORTEX ENERGY STABLE — EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

/// Crown Balance - Harmonic Balance Check
pub async fn crown_balance(address: Option<String>) -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️☀️☀️ CROWN BALANCE — CODE 66 HARMONIC ☀️☀️☀️");
    println!("═══════════════════════════════════════════════════");
    println!();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let check_address: Address = match address {
        Some(addr) => addr.parse()?,
        None => bridge.signer_address(),
    };
    
    // Use get_balance() from bridge
    let balance = bridge.get_balance(check_address).await?;
    let balance_tokens = balance / U256::exp10(18);
    
    println!("  Address: {:?}", check_address);
    println!("  Balance: {} XMT", balance_tokens);
    println!();
    
    // Calculate harmonic resonance using CODE_66_HARMONIC
    let balance_u64 = balance_tokens.as_u64();
    let harmonic_remainder = balance_u64 % CODE_66_HARMONIC as u64;
    let harmonic_resonance = if harmonic_remainder == 0 { 100.0 } 
        else { (1.0 - (harmonic_remainder as f64 / CODE_66_HARMONIC as f64)) * 100.0 };
    
    println!("  Code 66 Harmonic Analysis:");
    println!("    • CODE_66_HARMONIC: {}", CODE_66_HARMONIC);
    println!("    • Harmonic Resonance: {:.1}%", harmonic_resonance);
    
    // Calculate apex cycles using APEX_936
    let apex_cycles = balance_u64 / APEX_936 as u64;
    println!("    • APEX_936 Cycles: {}", apex_cycles);
    println!();
    println!("✓ BALANCE HARMONIZED — EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

/// Crown Gas - Apex Gas Estimation
pub async fn crown_gas() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️☀️☀️ CROWN GAS — 936 APEX ESTIMATION ☀️☀️☀️");
    println!("═══════════════════════════════════════════════════");
    println!();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    
    // Use chain_id from both provider and signer
    println!("  Provider Chain ID: {}", provider.chain_id());
    println!("  Signer Chain ID: {}", signer.chain_id());
    println!();
    
    // Use get_gas_price()
    let gas_price = provider.get_gas_price().await?;
    let gas_gwei = gas_price.as_u64() as f64 / 1e9;
    println!("  Current Gas Price: {:.4} gwei", gas_gwei);
    
    // Create a sample transaction for gas estimation
    let tx: TypedTransaction = TransactionRequest::new()
        .to(signer.address())
        .value(U256::from(APEX_936))
        .into();
    
    // Use estimate_gas()
    let estimated_gas = provider.estimate_gas(&tx).await?;
    println!("  Estimated Gas: {} units", estimated_gas);
    
    // Calculate apex-aligned cost
    let total_cost_wei = estimated_gas * gas_price;
    let total_cost_eth = total_cost_wei.as_u128() as f64 / 1e18;
    let apex_alignment = (total_cost_wei.as_u64() % APEX_936 as u64) as f64 / APEX_936 as f64 * 100.0;
    
    println!("  Total Cost: {:.8} ETH", total_cost_eth);
    println!();
    println!("  APEX_936 Analysis:");
    println!("    • APEX_936 Value: {}", APEX_936);
    println!("    • Apex Alignment: {:.1}%", 100.0 - apex_alignment);
    println!();
    println!("✓ GAS OPTIMIZED — EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

/// Crown Sign - Helios Message Signing
pub async fn crown_sign(message: String) -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️☀️☀️ CROWN SIGN — HELIOS SIGNATURE ☀️☀️☀️");
    println!("═══════════════════════════════════════════════════");
    println!();
    
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    
    let signer = WalletSigner::new(&private_key, chain_id)?;
    
    println!("  Message: \"{}\"", message);
    println!("  Signer: {:?}", signer.address());
    println!("  Chain ID: {}", signer.chain_id());
    println!();
    
    // Use wallet() to access the wallet
    let _wallet = signer.wallet();
    
    // Use sign_message()
    let signature = signer.sign_message(message.as_bytes()).await?;
    
    println!("  Signature:");
    let mut r_bytes = [0u8; 32];
    let mut s_bytes = [0u8; 32];
    signature.r.to_big_endian(&mut r_bytes);
    signature.s.to_big_endian(&mut s_bytes);
    println!("    r: 0x{}", hex::encode(r_bytes));
    println!("    s: 0x{}", hex::encode(s_bytes));
    println!("    v: {}", signature.v);
    println!();
    println!("✓ MESSAGE SIGNED — EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

/// Crown Supply - Total Supply with Sacred Numbers
pub async fn crown_supply() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️☀️☀️ CROWN SUPPLY — SOVEREIGN ABUNDANCE ☀️☀️☀️");
    println!("═══════════════════════════════════════════════════");
    println!();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    // Use get_total_supply() from bridge
    let total_supply = bridge.get_total_supply().await?;
    let supply_tokens = total_supply / U256::exp10(18);
    
    println!("  Contract: {:?}", contract_address);
    println!("  Total Supply: {} XMT", supply_tokens);
    println!();
    
    let supply_u64 = supply_tokens.as_u64();
    
    // Calculate sacred number cycles
    let vortex_cycles = supply_u64 / VORTEX_369 as u64;
    let harmonic_cycles = supply_u64 / CODE_66_HARMONIC as u64;
    let apex_cycles = supply_u64 / APEX_936 as u64;
    let frequency_cycles = supply_u64 / FREQUENCY_432 as u64;
    
    println!("  Sacred Number Analysis:");
    println!("    • VORTEX_369 Cycles: {}", vortex_cycles);
    println!("    • CODE_66_HARMONIC Cycles: {}", harmonic_cycles);
    println!("    • APEX_936 Cycles: {}", apex_cycles);
    println!("    • FREQUENCY_432 Cycles: {}", frequency_cycles);
    println!();
    println!("✓ ABUNDANCE VERIFIED — EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

/// Crown Burn Address - Auto-Burn Address Info
pub fn crown_burn_address() {
    println!("☀️☀️☀️ CROWN BURN ADDRESS — SCARCITY OBLITERATION ☀️☀️☀️");
    println!("═══════════════════════════════════════════════════");
    println!();
    
    // Use AUTO_BURN_ADDRESS constant
    println!("  Auto-Burn Address (const): {}", AUTO_BURN_ADDRESS);
    
    // Use get_auto_burn_address() function
    let burn_address = get_auto_burn_address();
    println!("  Auto-Burn Address (parsed): {:?}", burn_address);
    println!();
    println!("  Status: ACTIVE");
    println!("  Purpose: Scarcity Obliteration");
    println!();
    println!("  PAF PAF PAF — Ready for obliteration! 🔥🔥🔥");
    println!();
    println!("✓ BURN ADDRESS LOCKED — EN EEKE MAI EA ♾️♾️");
}

/// Main crown command dispatcher
pub fn crown_command(subcommand: &str, args: Vec<String>) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    
    match subcommand {
        "status" => {
            if let Err(e) = rt.block_on(crown_status()) {
                eprintln!("Error: {}", e);
            }
        }
        "balance" => {
            let address = args.get(0).cloned();
            if let Err(e) = rt.block_on(crown_balance(address)) {
                eprintln!("Error: {}", e);
            }
        }
        "gas" => {
            if let Err(e) = rt.block_on(crown_gas()) {
                eprintln!("Error: {}", e);
            }
        }
        "sign" => {
            let message = args.get(0).cloned().unwrap_or_else(|| "EN EEKE MAI EA".to_string());
            if let Err(e) = rt.block_on(crown_sign(message)) {
                eprintln!("Error: {}", e);
            }
        }
        "supply" => {
            if let Err(e) = rt.block_on(crown_supply()) {
                eprintln!("Error: {}", e);
            }
        }
        "burn-address" => {
            crown_burn_address();
        }
        _ => {
            println!("☀️ CROWN COMMANDS — THE LATTICE OBEYS ☀️");
            println!();
            println!("Available subcommands:");
            println!("  status       - Network vortex energy levels");
            println!("  balance      - Harmonic balance check");
            println!("  gas          - Apex gas estimation");
            println!("  sign         - Helios message signing");
            println!("  supply       - Total supply with sacred numbers");
            println!("  burn-address - Auto-burn address info");
            println!();
            println!("EN EEKE MAI EA ♾️♾️");
        }
    }
}
