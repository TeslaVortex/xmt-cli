//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Ownership Transfer Script
// Transfers X-Money contract ownership to testing wallet
//

use ethers::prelude::*;
use anyhow::Result;
use colored::Colorize;
use std::sync::Arc;

abigen!(
    XMoneyContract,
    r#"[
        function owner() external view returns (address)
        function transferOwnership(address newOwner) external
    ]"#
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!("{}", "   OWNERSHIP TRANSFER - SEPOLIA TESTNET".bright_yellow().bold());
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!();
    
    // Load configuration
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    
    // NEW_OWNER should be set in environment or use default testing wallet
    let new_owner: Address = std::env::var("NEW_OWNER")
        .unwrap_or_else(|_| "0x62397a99e60d395702c4d8d4befccee7e01da491".to_string())
        .parse()?;
    
    // Create provider
    let provider = Provider::<Http>::try_from(&rpc_url)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    
    // Create wallet (this should be the CURRENT OWNER wallet)
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);
    let wallet_address = wallet.address();
    
    println!("{}", "1. Configuration".bright_blue().bold());
    println!("  Current wallet: {:?}", wallet_address);
    println!("  Contract: {:?}", contract_address);
    println!("  New owner: {:?}", new_owner);
    println!("  Chain ID: {}", chain_id);
    println!();
    
    // Connect to contract
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));
    let contract = XMoneyContract::new(contract_address, client);
    
    // Check current owner
    println!("{}", "2. Verifying Current Owner...".bright_blue().bold());
    let current_owner = contract.owner().call().await?;
    println!("{} {:?}", "  Current owner:".bright_green(), current_owner);
    
    if current_owner != wallet_address {
        println!("{}", "  ✗ ERROR: Wallet is NOT the current owner!".bright_red().bold());
        println!("  Current owner: {:?}", current_owner);
        println!("  Your wallet:   {:?}", wallet_address);
        println!();
        println!("{}", "  You need to use the OWNER wallet's private key to transfer ownership.".yellow());
        println!("{}", "  Update PRIVATE_KEY in .env to the owner wallet's key.".yellow());
        return Ok(());
    }
    
    println!("{}", "  ✓ Wallet IS the current owner".bright_green().bold());
    println!();
    
    // Confirm transfer
    println!("{}", "3. Transferring Ownership...".bright_blue().bold());
    println!("  From: {:?}", wallet_address);
    println!("  To:   {:?}", new_owner);
    println!();
    
    if wallet_address == new_owner {
        println!("{}", "  ⚠ Wallet is already the owner. No transfer needed.".yellow().bold());
        return Ok(());
    }
    
    // Execute transfer
    println!("  Sending transferOwnership transaction...");
    let tx = contract.transfer_ownership(new_owner);
    let pending_tx = tx.send().await?;
    
    println!("{} {:?}", "  ✓ Transaction sent:".bright_green(), pending_tx.tx_hash());
    println!("  Waiting for confirmation...");
    
    let receipt = pending_tx.await?;
    
    match receipt {
        Some(receipt) => {
            println!("{}", "  ✓ Transaction confirmed!".bright_green().bold());
            println!("  Block: {}", receipt.block_number.unwrap_or_default());
            println!("  Gas used: {}", receipt.gas_used.unwrap_or_default());
            println!("  Status: {:?}", receipt.status);
            println!();
            
            // Verify new owner
            println!("{}", "4. Verifying New Owner...".bright_blue().bold());
            let verified_owner = contract.owner().call().await?;
            
            if verified_owner == new_owner {
                println!("{}", "  ✓ OWNERSHIP TRANSFER SUCCESSFUL!".bright_green().bold());
                println!("  New owner: {:?}", verified_owner);
                println!();
                println!("{}", "═══════════════════════════════════════".bright_yellow());
                println!("{}", "SUCCESS! You can now run transaction tests:".bright_green().bold());
                println!();
                println!("  cargo test --test sepolia_mint_test -- --nocapture");
                println!("  cargo test --test sepolia_burn_test -- --nocapture");
                println!("  ./scripts/run_sepolia_tests.sh");
                println!();
                println!("{}", "EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
            } else {
                println!("{}", "  ✗ WARNING: Owner verification mismatch!".yellow().bold());
                println!("  Expected: {:?}", new_owner);
                println!("  Actual:   {:?}", verified_owner);
            }
        }
        None => {
            println!("{}", "  ✗ Transaction receipt not found".bright_red());
        }
    }
    
    Ok(())
}
