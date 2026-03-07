//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// X-Money Blockchain Integration - Base Mainnet
// Real Quantum Abundance Transactions
//

use crate::pqc;
use crate::toroidal;
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;
use crate::bridge::XMoneyBridge;
use ethers::prelude::*;
use anyhow::Result;
use colored::Colorize;

pub async fn xmoney_integrate_async(action: &str) -> Result<()> {
    pqc::pqc_init();
    toroidal::toroidal_cycle();
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")
        .unwrap_or_else(|_| "https://mainnet.base.org".to_string());
    let chain_id: u64 = std::env::var("CHAIN_ID")
        .unwrap_or_else(|_| "8453".to_string())
        .parse()
        .unwrap_or(8453);
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")
        .unwrap_or_else(|_| "0x0000000000000000000000000000000000000000".to_string())
        .parse()?;
    
    // Initialize Web3 provider and signer
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    
    // Create bridge
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    match action {
        "mint" => {
            let amount = U256::from(100) * U256::exp10(18); // 100 tokens with 18 decimals
            let to = bridge.signer_address();
            
            println!("{}", "⚡ EXECUTING X-MONEY MINT ON BASE MAINNET".bright_yellow().bold());
            println!("{} {}", "  Amount:".bright_blue(), "100 tokens".bright_green().bold());
            println!("{} {}", "  Recipient:".bright_blue(), format!("{:?}", to).bright_cyan());
            println!();
            
            let receipt = bridge.mint(to, amount).await?;
            
            println!("{}", "✓ MINT SUCCESSFUL - ABUNDANCE FLOWS!".bright_green().bold());
            println!("{} {}", "  Transaction:".bright_blue(), format!("{:?}", receipt.transaction_hash).bright_cyan());
            println!("{} {}", "  Block:".bright_blue(), format!("{:?}", receipt.block_number).bright_magenta());
            println!("{} {}", "  Gas Used:".bright_blue(), format!("{:?}", receipt.gas_used).bright_yellow());
            println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
        },
        "burn" => {
            let amount = U256::from(50) * U256::exp10(18); // 50 tokens with 18 decimals
            
            println!("{}", "🔥 EXECUTING X-MONEY BURN - SCARCITY OBLITERATION".bright_red().bold());
            println!("{} {}", "  Amount:".bright_blue(), "50 tokens".bright_red().bold());
            println!();
            
            let receipt = bridge.burn(amount).await?;
            
            println!("{}", "✓ BURN SUCCESSFUL - PAF PAF PAF!".bright_green().bold());
            println!("{} {}", "  Transaction:".bright_blue(), format!("{:?}", receipt.transaction_hash).bright_cyan());
            println!("{} {}", "  Block:".bright_blue(), format!("{:?}", receipt.block_number).bright_magenta());
            println!("{} {}", "  Gas Used:".bright_blue(), format!("{:?}", receipt.gas_used).bright_yellow());
            println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
        },
        _ => {
            println!("Unknown X-Money action: {}", action);
            anyhow::bail!("Invalid action");
        },
    }
    
    Ok(())
}

// Synchronous wrapper for backward compatibility
pub fn xmoney_integrate(action: &str) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    
    match rt.block_on(xmoney_integrate_async(action)) {
        Ok(_) => println!("{}", "✓ X-Money integration complete with 936 apex".bright_green().bold()),
        Err(e) => {
            eprintln!("X-Money integration failed: {}", e);
            println!("{}", "⚠ Falling back to simulation mode...".yellow());
            
            // Fallback to simulation if blockchain call fails
            pqc::pqc_init();
            toroidal::toroidal_cycle();
            match action {
                "mint" => println!("{}", "✓ Simulated X-Money mint successful".bright_green()),
                "burn" => println!("{}", "✓ Simulated X-Money burn successful".bright_green()),
                _ => println!("Unknown action"),
            }
        }
    }
}
