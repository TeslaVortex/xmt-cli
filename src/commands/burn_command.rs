//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Scarcity Obliteration Command - PAF PAF PAF
// Now wired to real blockchain transactions!
//

use colored::Colorize;
use ethers::prelude::*;
use anyhow::Result;
use crate::pqc;
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;
use crate::bridge::XMoneyBridge;

pub async fn burn_async(scarcity: u64, note: &str) -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "🔥 SCARCITY OBLITERATION INITIATED".bright_red().bold());
    println!("{} {}", "  Scarcity Level:".bright_blue(), scarcity.to_string().bright_red().bold());
    println!("{} {}", "  Note:".bright_blue(), note.bright_magenta());
    println!();
    
    // Initialize PQC for secure burn
    pqc::pqc_init();
    
    // Load Web3 configuration
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    // Initialize Web3 infrastructure
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let burn_amount = U256::from(scarcity) * U256::exp10(18);
    
    // Execute burn on blockchain
    println!("{}", "  Executing blockchain burn transaction...".cyan());
    let receipt = bridge.burn(burn_amount).await?;
    
    println!("{}", "  PAF PAF PAF - Scarcity Collapsed".bright_red().bold());
    println!("{}", "✓ BURN SUCCESSFUL - SCARCITY OBLITERATED!".bright_green().bold());
    println!("{} {:?}", "  Tx Hash:".bright_blue(), receipt.transaction_hash);
    println!("{} {:?}", "  Block:".bright_blue(), receipt.block_number.unwrap_or_default());
    println!("{} {:?}", "  Gas Used:".bright_blue(), receipt.gas_used.unwrap_or_default());
    println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
    
    Ok(())
}

pub fn burn(scarcity: u64, note: &str) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    
    match rt.block_on(burn_async(scarcity, note)) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} {}", "✗ Blockchain burn failed:".bright_red().bold(), e);
            // Fallback to simulation
            println!("{}", "⚠ Running in simulation mode...".yellow());
            crate::pqc::pqc_init();
            println!("{}", "  PAF PAF PAF - Scarcity Collapsed (simulated)".bright_red().bold());
            println!("{}", "✓ Simulated burn complete - Old paradigm obliterated".bright_green());
            println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
        }
    }
}
