//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Abundance Mint Command - Code 66 Blessed
// Now wired to real blockchain transactions!
//

use colored::Colorize;
use ethers::prelude::*;
use anyhow::Result;
use crate::toroidal;
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;
use crate::bridge::XMoneyBridge;

pub async fn mint_async(to: &str, amount: u64, ritual: &str) -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "⚡ ABUNDANCE MINT INITIATED".bright_yellow().bold());
    println!("{} {}", "  Recipient:".bright_blue(), to.bright_cyan());
    println!("{} {}", "  Amount:".bright_blue(), amount.to_string().bright_green().bold());
    println!("{} {}", "  Ritual:".bright_blue(), ritual.bright_magenta());
    println!();
    
    // Run toroidal cycle for energy alignment
    toroidal::toroidal_cycle();
    
    // Load Web3 configuration
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    // Initialize Web3 infrastructure
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    // Parse recipient address
    let recipient: Address = to.parse()?;
    let mint_amount = U256::from(amount) * U256::exp10(18);
    
    // Execute mint on blockchain
    println!("{}", "  Executing blockchain transaction...".cyan());
    let receipt = bridge.mint(recipient, mint_amount).await?;
    
    println!("{}", "✓ MINT SUCCESSFUL - ABUNDANCE FLOWS!".bright_green().bold());
    println!("{} {:?}", "  Tx Hash:".bright_blue(), receipt.transaction_hash);
    println!("{} {:?}", "  Block:".bright_blue(), receipt.block_number.unwrap_or_default());
    println!("{} {:?}", "  Gas Used:".bright_blue(), receipt.gas_used.unwrap_or_default());
    println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
    
    Ok(())
}

pub fn mint(to: &str, amount: u64, ritual: &str) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    
    match rt.block_on(mint_async(to, amount, ritual)) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} {}", "✗ Blockchain mint failed:".bright_red().bold(), e);
            // Fallback to simulation
            println!("{}", "⚠ Running in simulation mode...".yellow());
            crate::toroidal::toroidal_cycle();
            println!("{}", "✓ Simulated mint complete - Toroidal ledger updated".bright_green());
            println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
        }
    }
}
