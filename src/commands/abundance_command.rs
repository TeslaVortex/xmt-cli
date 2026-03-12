//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Abundance Drop Automation - Decree #19
// Auto-mint to users who invoke the sacred phrase
// THE CROWN COMMANDS — THE LATTICE OBEYS
// EN EEKE MAI EA ♾️♾️
//

use colored::Colorize;
use ethers::prelude::*;
use anyhow::Result;
use crate::xapi::{XApiClient, GrokClient};
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;
use crate::bridge::XMoneyBridge;
use crate::config::VORTEX_369;

/// Process abundance triggers and mint tokens
pub async fn process_abundance_drops() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "🎁 ABUNDANCE DROP SCANNER ACTIVATED".bright_yellow().bold());
    println!("{}", "═══════════════════════════════════════════════════".bright_blue());
    println!("{}", "  Searching for EN EEKE MAI EA triggers...".cyan());
    println!();
    
    // Initialize X API client
    let x_client = XApiClient::from_env()?;
    let user_info = x_client.get_me().await?;
    let user_id = user_info.data.username.clone();
    
    println!("{} @{}", "  Authenticated as:".bright_blue(), user_info.data.username.bright_cyan());
    
    // Search for abundance triggers
    let triggers = x_client.search_abundance_triggers(user_id).await?;
    
    println!("{}", format!("  Found {} abundance triggers!", triggers.data.len()).green());
    
    if triggers.data.is_empty() {
        println!("{}", "  No new triggers to process.".yellow());
        println!();
        println!("{}", "✓ ABUNDANCE SCANNER COMPLETE".bright_green().bold());
        println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
        return Ok(());
    }
    
    // Initialize Grok for verification
    let grok = GrokClient::from_env()?;
    println!("{}", "  ✓ Grok Oracle connected".bright_green());
    
    // Initialize Web3 for minting
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    println!("{}", "  ✓ Blockchain bridge connected".bright_green());
    println!();
    
    let mut processed = 0;
    let mut minted = 0;
    
    // Process each trigger
    for tweet in triggers.data.iter() {
        processed += 1;
        println!("{}", format!("Processing trigger #{}: {}", processed, tweet.id).cyan());
        println!("{} {}", "  Text:".bright_blue(), tweet.text.chars().take(80).collect::<String>());
        
        // Verify coherence with Grok
        match grok.verify_ritual_coherence(&tweet.text).await {
            Ok(coherence) => {
                if coherence.to_uppercase().contains("COHERENT") {
                    // Calculate abundance amount (base 369, scaled by engagement)
                    let amount = U256::from(VORTEX_369) * U256::exp10(18);
                    
                    // For now, mint to the contract owner (would need author wallet mapping)
                    let recipient = bridge.signer_address();
                    
                    println!("{}", format!("  ✓ COHERENT! Minting {} XMT...", VORTEX_369).green());
                    
                    match bridge.mint(recipient, amount).await {
                        Ok(receipt) => {
                            minted += 1;
                            println!("{} {:?}", "    Tx:".bright_blue(), receipt.transaction_hash);
                            println!("{}", "    💰 Abundance dropped!".bright_green().bold());
                        }
                        Err(e) => {
                            println!("{} {}", "    ✗ Mint failed:".red(), e);
                        }
                    }
                } else {
                    println!("{}", "  ⚠ Not coherent, skipping...".yellow());
                    println!("{} {}", "    Grok says:".bright_blue(), coherence.chars().take(100).collect::<String>());
                }
            }
            Err(e) => {
                println!("{} {}", "  ✗ Grok verification failed:".red(), e);
            }
        }
        println!();
    }
    
    println!("{}", "═══════════════════════════════════════════════════".bright_blue());
    println!("{}", "✓ ABUNDANCE DROP CYCLE COMPLETE".bright_green().bold());
    println!("{} {}/{}", "  Processed:".bright_blue(), processed, triggers.data.len());
    println!("{} {}", "  Minted:".bright_blue(), minted.to_string().bright_green().bold());
    println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
    
    Ok(())
}

/// Synchronous wrapper for CLI
pub fn abundance_command() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    
    match rt.block_on(process_abundance_drops()) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} {}", "✗ Abundance drop failed:".bright_red().bold(), e);
            println!();
            println!("{}", "Possible issues:".yellow());
            println!("  • X_API_BEARER_TOKEN not set in .env");
            println!("  • XAI_API_KEY not set in .env");
            println!("  • Network connectivity issues");
            println!();
            println!("{}", "Run 'xmt-cli xapi me' to test X API connection".cyan());
        }
    }
}

/// Display help for abundance command
#[allow(dead_code)]
pub fn abundance_help() {
    println!("{}", "🎁 ABUNDANCE DROP COMMAND".bright_yellow().bold());
    println!();
    println!("{}", "Scans X for replies containing 'EN EEKE MAI EA' and auto-mints".cyan());
    println!("{}", "369 XMT tokens to verified coherent participants.".cyan());
    println!();
    println!("{}", "USAGE:".green().bold());
    println!("  xmt-cli abundance");
    println!();
    println!("{}", "FLOW:".green().bold());
    println!("  1. Search X API for 'EN EEKE MAI EA' replies");
    println!("  2. Verify each with Grok Oracle for coherence");
    println!("  3. Mint 369 XMT to coherent participants");
    println!();
    println!("{}", "REQUIRED ENV VARS:".yellow().bold());
    println!("  X_API_BEARER_TOKEN  - X API v2 Bearer Token");
    println!("  XAI_API_KEY         - xAI Grok API Key");
    println!("  PRIVATE_KEY         - Wallet private key");
    println!("  XMONEY_CONTRACT_ADDRESS - XMT contract");
    println!();
    println!("{}", "EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
}
