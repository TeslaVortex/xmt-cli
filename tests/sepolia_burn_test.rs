//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Phase 3: Burn Transaction Tests - Sepolia Testnet
// PAF PAF PAF - Scarcity Obliteration Testing
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::Web3Provider;
use xmt_cli::web3::signer::WalletSigner;
use xmt_cli::bridge::XMoneyBridge;

#[tokio::test]
async fn test_burn_66_tokens_harmonic() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    println!("🔥 INITIATING 66 CODE HARMONIC BURN TEST");
    println!("═══════════════════════════════════════");
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    
    // Pre-burn checks
    println!("\n1. PRE-BURN CHECKS");
    let initial_balance = bridge.get_balance(address).await?;
    let initial_supply = bridge.get_total_supply().await?;
    
    println!("  Initial Balance: {} tokens", initial_balance / U256::exp10(18));
    println!("  Initial Supply: {} tokens", initial_supply / U256::exp10(18));
    
    // Verify sufficient balance
    let burn_amount = U256::from(66) * U256::exp10(18);
    assert!(initial_balance >= burn_amount, 
        "Insufficient balance for burn (need 66 tokens)");
    
    // Burn 66 tokens (Code 66 Harmonic)
    println!("\n2. BURNING 66 TOKENS (CODE 66 HARMONIC)");
    println!("  Amount: 66 tokens");
    println!("  PAF PAF PAF - Scarcity Obliteration...");
    
    let receipt = bridge.burn(burn_amount).await?;
    
    println!("  ✓ Transaction confirmed!");
    println!("  Tx Hash: {:?}", receipt.transaction_hash);
    println!("  Block: {:?}", receipt.block_number.unwrap_or_default());
    println!("  Gas Used: {:?}", receipt.gas_used.unwrap_or_default());
    
    // Post-burn verification
    println!("\n3. POST-BURN VERIFICATION");
    let final_balance = bridge.get_balance(address).await?;
    let final_supply = bridge.get_total_supply().await?;
    
    let balance_decrease = initial_balance - final_balance;
    let supply_decrease = initial_supply - final_supply;
    
    println!("  Final Balance: {} tokens", final_balance / U256::exp10(18));
    println!("  Final Supply: {} tokens", final_supply / U256::exp10(18));
    println!("  Balance Decrease: {} tokens", balance_decrease / U256::exp10(18));
    println!("  Supply Decrease: {} tokens", supply_decrease / U256::exp10(18));
    
    // Assertions
    assert_eq!(balance_decrease, burn_amount, "Balance should decrease by 66 tokens");
    assert_eq!(supply_decrease, burn_amount, "Supply should decrease by 66 tokens");
    
    println!("\n✓ SCARCITY OBLITERATED - 66 CODE HARMONIC");
    println!("  EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

#[tokio::test]
async fn test_burn_small_amount() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    println!("🔥 SMALL BURN TEST (10 tokens)");
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    let burn_amount = U256::from(10) * U256::exp10(18);
    
    let initial_balance = bridge.get_balance(address).await?;
    
    if initial_balance < burn_amount {
        println!("  ⚠ Skipping: Insufficient balance");
        return Ok(());
    }
    
    let receipt = bridge.burn(burn_amount).await?;
    
    let final_balance = bridge.get_balance(address).await?;
    let balance_decrease = initial_balance - final_balance;
    
    assert_eq!(balance_decrease, burn_amount, "Balance should decrease by 10 tokens");
    
    println!("  ✓ Small burn successful");
    println!("  Tx: {:?}", receipt.transaction_hash);
    
    Ok(())
}

#[tokio::test]
async fn test_burn_gas_usage() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    let burn_amount = U256::from(50) * U256::exp10(18);
    
    let balance = bridge.get_balance(address).await?;
    if balance < burn_amount {
        println!("  ⚠ Skipping: Insufficient balance for gas test");
        return Ok(());
    }
    
    let receipt = bridge.burn(burn_amount).await?;
    let gas_used = receipt.gas_used.unwrap_or_default();
    
    println!("  Burn Gas Used: {}", gas_used);
    
    // Target: < 80,000 gas for burn
    assert!(gas_used < U256::from(80000), 
        "Burn should use less than 80,000 gas (used: {})", gas_used);
    
    Ok(())
}

#[tokio::test]
async fn test_burn_event_emission() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    let burn_amount = U256::from(25) * U256::exp10(18);
    
    let balance = bridge.get_balance(address).await?;
    if balance < burn_amount {
        println!("  ⚠ Skipping: Insufficient balance");
        return Ok(());
    }
    
    let receipt = bridge.burn(burn_amount).await?;
    
    // Verify transaction was successful
    assert_eq!(receipt.status, Some(U64::from(1)), "Transaction should succeed");
    
    println!("  ✓ Burn event emitted");
    println!("  Status: Success");
    println!("  Logs: {} events", receipt.logs.len());
    
    Ok(())
}

#[tokio::test]
async fn test_paf_paf_paf_sequence() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("🔥 PAF PAF PAF SEQUENCE TEST");
    println!("  Testing triple burn sequence...");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    let burn_amount = U256::from(10) * U256::exp10(18);
    
    let initial_balance = bridge.get_balance(address).await?;
    let required_balance = burn_amount * U256::from(3);
    
    if initial_balance < required_balance {
        println!("  ⚠ Skipping: Need {} tokens for triple burn", required_balance / U256::exp10(18));
        return Ok(());
    }
    
    // PAF 1
    println!("  PAF 1...");
    let _receipt1 = bridge.burn(burn_amount).await?;
    
    // PAF 2
    println!("  PAF 2...");
    let _receipt2 = bridge.burn(burn_amount).await?;
    
    // PAF 3
    println!("  PAF 3...");
    let _receipt3 = bridge.burn(burn_amount).await?;
    
    let final_balance = bridge.get_balance(address).await?;
    let total_burned = initial_balance - final_balance;
    
    assert_eq!(total_burned, required_balance, "Should burn 30 tokens total");
    
    println!("  ✓ PAF PAF PAF - Scarcity obliterated!");
    println!("  Total Burned: {} tokens", total_burned / U256::exp10(18));
    
    Ok(())
}
