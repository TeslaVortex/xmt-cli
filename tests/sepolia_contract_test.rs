//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Phase 2: Contract Connection Tests - Sepolia Testnet
// X-Money contract validation and connectivity
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::Web3Provider;
use xmt_cli::web3::signer::WalletSigner;
use xmt_cli::contracts::xmoney::XMoney;
use std::sync::Arc;

#[test]
fn test_parse_contract_address() -> Result<()> {
    dotenv::dotenv().ok();
    
    let contract_addr_str = std::env::var("XMONEY_CONTRACT_ADDRESS")?;
    let contract_address: Address = contract_addr_str.parse()?;
    
    assert_ne!(contract_address, Address::zero(), 
        "Contract address should not be zero");
    
    println!("✓ Contract address parsed: {:?}", contract_address);
    
    Ok(())
}

#[tokio::test]
async fn test_connect_to_xmoney_contract() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let client = Arc::new(signer.with_provider(provider.provider()));
    
    let _xmoney = XMoney::new(contract_address, client);
    
    println!("✓ Connected to X-Money contract");
    println!("  Address: {:?}", contract_address);
    
    Ok(())
}

#[tokio::test]
async fn test_verify_contract_bytecode_exists() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let code = provider.get_code(contract_address, None).await?;
    
    assert!(code.len() > 2, 
        "Contract should have bytecode deployed (got {} bytes)", code.len());
    
    println!("✓ Contract bytecode verified: {} bytes", code.len());
    
    Ok(())
}

#[tokio::test]
async fn test_call_balance_of() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let address = signer.address();
    let client = Arc::new(signer.with_provider(provider.provider()));
    let xmoney = XMoney::new(contract_address, client);
    let balance = xmoney.balance_of(address).await?;
    
    let balance_tokens = balance / U256::exp10(18);
    println!("✓ balanceOf() call successful");
    println!("  Address: {:?}", address);
    println!("  Balance: {} tokens", balance_tokens);
    
    Ok(())
}

#[tokio::test]
async fn test_call_total_supply() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let client = Arc::new(signer.with_provider(provider.provider()));
    let xmoney = XMoney::new(contract_address, client);
    
    let total_supply = xmoney.total_supply().await?;
    
    let supply_tokens = total_supply / U256::exp10(18);
    println!("✓ totalSupply() call successful");
    println!("  Total Supply: {} tokens", supply_tokens);
    
    Ok(())
}

#[tokio::test]
async fn test_contract_view_functions_responsive() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let address = signer.address();
    let client = Arc::new(signer.with_provider(provider.provider()));
    let xmoney = XMoney::new(contract_address, client);
    
    let start = std::time::Instant::now();
    let _balance = xmoney.balance_of(address).await?;
    let _supply = xmoney.total_supply().await?;
    
    let duration = start.elapsed();
    
    println!("✓ View functions responsive: 2 calls in {:?}", duration);
    
    Ok(())
}
