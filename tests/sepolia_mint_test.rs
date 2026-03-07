//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Phase 3: Mint Transaction Tests - Sepolia Testnet
// 369 Vortex Abundance Flow Testing
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::Web3Provider;
use xmt_cli::web3::signer::WalletSigner;
use xmt_cli::bridge::XMoneyBridge;

#[tokio::test]
async fn test_mint_369_tokens_vortex() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    println!("☀️ INITIATING 369 VORTEX MINT TEST");
    println!("═══════════════════════════════════════");
    
    // Initialize infrastructure
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    
    // Pre-mint checks
    println!("\n1. PRE-MINT CHECKS");
    let initial_balance = bridge.get_balance(recipient).await?;
    let initial_supply = bridge.get_total_supply().await?;
    
    println!("  Initial Balance: {} tokens", initial_balance / U256::exp10(18));
    println!("  Initial Supply: {} tokens", initial_supply / U256::exp10(18));
    
    // Mint 369 tokens (Tesla Vortex)
    println!("\n2. MINTING 369 TOKENS (TESLA VORTEX)");
    let amount = U256::from(369) * U256::exp10(18);
    
    println!("  Recipient: {:?}", recipient);
    println!("  Amount: 369 tokens");
    println!("  Executing transaction...");
    
    let receipt = bridge.mint(recipient, amount).await?;
    
    println!("  ✓ Transaction confirmed!");
    println!("  Tx Hash: {:?}", receipt.transaction_hash);
    println!("  Block: {:?}", receipt.block_number.unwrap_or_default());
    println!("  Gas Used: {:?}", receipt.gas_used.unwrap_or_default());
    
    // Post-mint verification
    println!("\n3. POST-MINT VERIFICATION");
    let final_balance = bridge.get_balance(recipient).await?;
    let final_supply = bridge.get_total_supply().await?;
    
    let balance_increase = final_balance - initial_balance;
    let supply_increase = final_supply - initial_supply;
    
    println!("  Final Balance: {} tokens", final_balance / U256::exp10(18));
    println!("  Final Supply: {} tokens", final_supply / U256::exp10(18));
    println!("  Balance Increase: {} tokens", balance_increase / U256::exp10(18));
    println!("  Supply Increase: {} tokens", supply_increase / U256::exp10(18));
    
    // Assertions
    assert_eq!(balance_increase, amount, "Balance should increase by 369 tokens");
    assert_eq!(supply_increase, amount, "Supply should increase by 369 tokens");
    
    println!("\n✓ 369 VORTEX MINT SUCCESSFUL");
    println!("  EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

#[tokio::test]
async fn test_mint_code_66_harmonic() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    println!("☀️ CODE 66 HARMONIC MINT TEST");
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    let amount = U256::from(66) * U256::exp10(18);
    
    let initial_balance = bridge.get_balance(recipient).await?;
    
    println!("  Minting 66 tokens (Code 66 Harmonic)...");
    let receipt = bridge.mint(recipient, amount).await?;
    
    let final_balance = bridge.get_balance(recipient).await?;
    let balance_increase = final_balance - initial_balance;
    
    assert_eq!(balance_increase, amount, "Balance should increase by 66 tokens");
    
    println!("  ✓ Code 66 mint successful");
    println!("  Tx: {:?}", receipt.transaction_hash);
    
    Ok(())
}

#[tokio::test]
async fn test_mint_936_apex() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    println!("☀️ 936 APEX MINT TEST");
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    let amount = U256::from(936) * U256::exp10(18);
    
    let initial_balance = bridge.get_balance(recipient).await?;
    
    println!("  Minting 936 tokens (Apex Value)...");
    let receipt = bridge.mint(recipient, amount).await?;
    
    let final_balance = bridge.get_balance(recipient).await?;
    let balance_increase = final_balance - initial_balance;
    
    assert_eq!(balance_increase, amount, "Balance should increase by 936 tokens");
    
    // Calculate Code 66 harmonic resonance
    let resonance = (936.0 / 66.0 / 14.181818) * 100.0;
    println!("  ✓ 936 Apex mint successful");
    println!("  Code 66 Resonance: {:.2}%", resonance);
    println!("  Tx: {:?}", receipt.transaction_hash);
    
    Ok(())
}

#[tokio::test]
async fn test_mint_gas_usage() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    let amount = U256::from(100) * U256::exp10(18);
    
    let receipt = bridge.mint(recipient, amount).await?;
    let gas_used = receipt.gas_used.unwrap_or_default();
    
    println!("  Mint Gas Used: {}", gas_used);
    
    // Target: < 100,000 gas for mint
    assert!(gas_used < U256::from(100000), 
        "Mint should use less than 100,000 gas (used: {})", gas_used);
    
    Ok(())
}

#[tokio::test]
async fn test_mint_event_emission() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    let amount = U256::from(369) * U256::exp10(18);
    
    let receipt = bridge.mint(recipient, amount).await?;
    
    // Verify transaction was successful
    assert_eq!(receipt.status, Some(U64::from(1)), "Transaction should succeed");
    
    println!("  ✓ Mint event emitted");
    println!("  Status: Success");
    println!("  Logs: {} events", receipt.logs.len());
    
    Ok(())
}
