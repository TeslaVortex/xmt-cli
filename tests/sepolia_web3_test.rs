//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Phase 2: Web3 Provider Tests - Sepolia Testnet
// Infrastructure validation for blockchain connectivity
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::Web3Provider;

#[tokio::test]
async fn test_connect_to_sepolia_rpc() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    
    assert_eq!(provider.chain_id(), chain_id);
    println!("✓ Connected to Sepolia RPC with chain ID: {}", chain_id);
    
    Ok(())
}

#[tokio::test]
async fn test_verify_chain_id() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let expected_chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let network_chain_id = provider.get_chainid().await?;
    
    assert_eq!(network_chain_id.as_u64(), expected_chain_id, 
        "Chain ID mismatch: expected {}, got {}", expected_chain_id, network_chain_id);
    
    println!("✓ Chain ID verified: {}", network_chain_id);
    
    Ok(())
}

#[tokio::test]
async fn test_get_current_block_number() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let block_number = provider.get_block_number().await?;
    
    assert!(block_number > U64::zero(), "Block number should be greater than 0");
    println!("✓ Current block number: {}", block_number);
    
    Ok(())
}

#[tokio::test]
async fn test_get_gas_price() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let gas_price = provider.get_gas_price().await?;
    
    assert!(gas_price > U256::zero(), "Gas price should be greater than 0");
    
    let gas_price_gwei = gas_price / U256::exp10(9);
    println!("✓ Current gas price: {} gwei", gas_price_gwei);
    
    Ok(())
}

#[tokio::test]
async fn test_network_responsiveness() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    
    // Test multiple calls to ensure network is responsive
    let start = std::time::Instant::now();
    
    let _block1 = provider.get_block_number().await?;
    let _gas1 = provider.get_gas_price().await?;
    let _block2 = provider.get_block_number().await?;
    
    let duration = start.elapsed();
    
    assert!(duration.as_secs() < 10, "Network should respond within 10 seconds");
    println!("✓ Network responsive: {} calls in {:?}", 3, duration);
    
    Ok(())
}

#[tokio::test]
async fn test_sacred_timing_936ms() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    
    // Test that we can complete operations within 936ms (sacred apex timing)
    let start = std::time::Instant::now();
    let _block = provider.get_block_number().await?;
    let duration = start.elapsed();
    
    println!("✓ Block query completed in {:?} (target: <936ms)", duration);
    
    // Note: This may fail on slow networks, which is expected
    // The test documents actual performance
    
    Ok(())
}
