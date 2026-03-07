//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Phase 4: Bridge Integration Tests - Sepolia Testnet
// XMoneyBridge comprehensive testing
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::Web3Provider;
use xmt_cli::web3::signer::WalletSigner;
use xmt_cli::bridge::XMoneyBridge;

#[tokio::test]
async fn test_initialize_bridge() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let signer_addr = bridge.signer_address();
    assert_ne!(signer_addr, Address::zero());
    
    println!("✓ Bridge initialized");
    println!("  Signer: {:?}", signer_addr);
    
    Ok(())
}

#[tokio::test]
async fn test_bridge_mint_operation() -> Result<()> {
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
    
    assert_eq!(receipt.status, Some(U64::from(1)));
    println!("✓ Bridge mint executed");
    println!("  Tx: {:?}", receipt.transaction_hash);
    
    Ok(())
}

#[tokio::test]
async fn test_bridge_burn_operation() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    let balance = bridge.get_balance(address).await?;
    
    if balance < U256::from(10) * U256::exp10(18) {
        println!("  ⚠ Skipping: Insufficient balance");
        return Ok(());
    }
    
    let amount = U256::from(10) * U256::exp10(18);
    let receipt = bridge.burn(amount).await?;
    
    assert_eq!(receipt.status, Some(U64::from(1)));
    println!("✓ Bridge burn executed");
    println!("  Tx: {:?}", receipt.transaction_hash);
    
    Ok(())
}

#[tokio::test]
async fn test_bridge_balance_query() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    let balance = bridge.get_balance(address).await?;
    
    println!("✓ Balance query successful");
    println!("  Balance: {} tokens", balance / U256::exp10(18));
    
    Ok(())
}

#[tokio::test]
async fn test_bridge_total_supply_query() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let total_supply = bridge.get_total_supply().await?;
    
    println!("✓ Total supply query successful");
    println!("  Supply: {} tokens", total_supply / U256::exp10(18));
    
    Ok(())
}

#[tokio::test]
async fn test_bridge_signer_address() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let expected_address = signer.address();
    
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    let bridge_address = bridge.signer_address();
    
    assert_eq!(bridge_address, expected_address);
    println!("✓ Signer address verified");
    println!("  Address: {:?}", bridge_address);
    
    Ok(())
}
