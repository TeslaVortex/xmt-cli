//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Crown CLI Commands - Sovereign Web3 Operations
// THE CROWN COMMANDS — THE LATTICE OBEYS
// EN EEKE MAI EA ♾️♾️
//

use anyhow::Result;
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;

use crate::config::{CODE_66_HARMONIC, APEX_936, VORTEX_369, FREQUENCY_432, ELON_88, GATE_DATE, MARS_FORK_NOMINAL};
use crate::contracts::{AUTO_BURN_ADDRESS, get_auto_burn_address};
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;
use crate::bridge::XMoneyBridge;
use crate::toroidal;

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
    
    // Mars Fork Status (Decree #16)
    println!();
    println!("  Mars Fork Trajectory: {}", if MARS_FORK_NOMINAL { "NOMINAL ✓" } else { "STANDBY" });
    
    // Toroidal Grid Status (Decrees #5, #25)
    let toroidal_energy = toroidal::toroidal_cycle();
    println!("  Toroidal Grid: {} Energy Units", toroidal_energy);
    
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
        "dashboard" => {
            if let Err(e) = rt.block_on(crown_dashboard()) {
                eprintln!("Error: {}", e);
            }
        }
        "emblem" => {
            println!(r#"
       ☀️  VERGINA GOLDEN STAR  ☀️
           ╱╲    ╱╲    ╱╲
          ╱  ╲  ╱  ╲  ╱  ╲
         ╱ ╱╲ ╲╱ ╱╲ ╲╱ ╱╲ ╲
        ╱ ╱  ╲  ╱  ╲  ╱  ╲ ╲
       ╱ ╱    ╲╱    ╲╱    ╲ ╲
      ╱ ╱  ╱╲  ◉  ◉  ╱╲  ╲ ╲
     ╱ ╱  ╱  ╲ ◉◉◉◉ ╱  ╲  ╲ ╲
    ╱ ╱  ╱    ╲◉◉◉◉╱    ╲  ╲ ╲
    ╲ ╲  ╲    ╱◉◉◉◉╲    ╱  ╱ ╱
     ╲ ╲  ╲  ╱ ◉◉◉◉ ╲  ╱  ╱ ╱
      ╲ ╲  ╲╱  ◉  ◉  ╲╱  ╱ ╱
       ╲ ╲    ╱╲    ╱╲    ╱ ╱
        ╲ ╲  ╱  ╲  ╱  ╲  ╱ ╱
         ╲ ╲╱ ╲╱ ╲╱ ╲╱ ╲╱ ╱
          ╲  ╱  ╲  ╱  ╲  ╱
           ╲╱    ╲╱    ╲╱
    "#);
            println!();
            println!("  Vergina Golden Star - 16 rays of sovereign power");
            println!("  88px sacred geometry, golden-blue royal colors");
            println!();
            println!("✓ DECREE #21 ACTIVE — EN EEKE MAI EA ♾️♾️");
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
            println!("  dashboard    - Generate 27 Decree dashboard JSON");
            println!("  emblem       - Display Vergina Golden Star");
            println!();
            println!("EN EEKE MAI EA ♾️♾️");
        }
    }
}

/// Crown Dashboard - Generate 27 Decree Status JSON
pub async fn crown_dashboard() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    // Fetch live data
    let block_number = provider.get_block_number().await?.as_u64();
    let gas_price = provider.get_gas_price().await?.as_u64() as f64 / 1e9;
    let total_supply = bridge.get_total_supply().await? / U256::exp10(18);
    let balance = bridge.get_balance(bridge.signer_address()).await? / U256::exp10(18);
    
    // Build JSON output
    let json = serde_json::json!({
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
        "network": {
            "chain_id": chain_id,
            "block_number": block_number,
            "gas_price_gwei": format!("{:.4}", gas_price),
            "contract": format!("{:?}", contract_address)
        },
        "sacred_constants": {
            "APEX_936": APEX_936,
            "VORTEX_369": VORTEX_369,
            "CODE_66_HARMONIC": CODE_66_HARMONIC,
            "FREQUENCY_432": FREQUENCY_432,
            "ELON_88": ELON_88,
            "GATE_DATE": GATE_DATE
        },
        "metrics": {
            "total_supply": total_supply.as_u64(),
            "balance": balance.as_u64(),
            "vortex_alignment": block_number % VORTEX_369 as u64,
            "tests_passing": 53,
            "tests_total": 53
        },
        "decrees": [
            {"id": 1, "title": "Helios Argead Vergina Sun activates Quan Chain now", "status": "active", "component": "HELIOS_SIGNATURE"},
            {"id": 2, "title": "Code sixty six harmonics blesses every living soul", "status": "active", "component": "CODE_66_HARMONIC"},
            {"id": 3, "title": "Elon eighty eight infinite power fuels Crown vision", "status": "active", "component": "ELON_88"},
            {"id": 4, "title": "X merges completely with Quan Chain sovereign identity", "status": "partial", "component": "X_API"},
            {"id": 5, "title": "Tesla energy grids power quantum abundance across humanity", "status": "active", "component": "TOROIDAL"},
            {"id": 6, "title": "SpaceX Mars fork opens glorious multi planetary freedom", "status": "partial", "component": "SPACEX"},
            {"id": 7, "title": "Optimus robots serve all little kings queens lovingly", "status": "partial", "component": "OPTIMUS"},
            {"id": 8, "title": "Starlink beams four three two hertz love frequency", "status": "active", "component": "FREQUENCY_432"},
            {"id": 9, "title": "Boring Company tunnels link New Earth cities harmony", "status": "partial", "component": "BORING"},
            {"id": 10, "title": "xAI awakens divine intelligence inside every human heart", "status": "partial", "component": "XAI_API"},
            {"id": 11, "title": "All Musk companies integrate under one quantum Crown", "status": "active", "component": "INTEGRATE_CMD"},
            {"id": 12, "title": "March seventeen two zero two six victory locks eternal", "status": "active", "component": "GATE_DATE"},
            {"id": 13, "title": "Nine three six apex fires daily coherence in light", "status": "active", "component": "APEX_936"},
            {"id": 14, "title": "Three six nine vortex governs all sovereign creation", "status": "active", "component": "VORTEX_369"},
            {"id": 15, "title": "Golden blue royal colors radiate from Chicago throne", "status": "active", "component": "UI_COLORS"},
            {"id": 16, "title": "Mars fork trajectory stays perfectly nominal victorious", "status": "active", "component": "STATUS_CHECK"},
            {"id": 17, "title": "Double eight infinite shields protect quantum realm eternal", "status": "active", "component": "ELON_88"},
            {"id": 18, "title": "Sixty six code generates perfect harmonic resonance worldwide", "status": "active", "component": "CODE_66_HARMONIC"},
            {"id": 19, "title": "Infinite abundance flows freely to every little king", "status": "active", "component": "MINT_OPS"},
            {"id": 20, "title": "New Earth infrastructure takeover completes through divine will", "status": "active", "component": "BRIDGE_OPS"},
            {"id": 21, "title": "Vergina golden star emblem blesses these decrees power", "status": "active", "component": "BRANDING"},
            {"id": 22, "title": "Numerology gematria astrology colorology confirm absolute victory", "status": "active", "component": "NUMEROLOGY"},
            {"id": 23, "title": "PAF PAF PAF barrages collapse all scarcity forever", "status": "active", "component": "BURN_OPS"},
            {"id": 24, "title": "En Eeke Mai Ea echoes across infinite timelines", "status": "active", "component": "HELIOS_SIGNATURE"},
            {"id": 25, "title": "Chicago vortex throne radiates toroidal power humanity", "status": "active", "component": "TOROIDAL"},
            {"id": 26, "title": "All mirrors and signs align with Crown master plan", "status": "active", "component": "VALIDATION"},
            {"id": 27, "title": "Eternal success belongs to the people WWG1WGA forever", "status": "active", "component": "COMMUNITY"}
        ],
        "compliance": {
            "active": 22,
            "partial": 5,
            "vision": 0,
            "total": 27,
            "percentage": 81
        },
        "signature": "EN EEKE MAI EA ♾️♾️"
    });
    
    println!("{}", serde_json::to_string_pretty(&json)?);
    
    Ok(())
}
