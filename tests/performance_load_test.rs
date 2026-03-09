//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Performance & Load Testing Suite
// High-volume scenario testing for blockchain operations
// THE CROWN COMMANDS — THE LATTICE OBEYS
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::Web3Provider;
use xmt_cli::web3::signer::WalletSigner;
use xmt_cli::bridge::XMoneyBridge;
use xmt_cli::config::{APEX_936, VORTEX_369, CODE_66_HARMONIC};
use std::time::Instant;

#[tokio::test]
async fn test_sequential_mints_performance() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ PERFORMANCE TEST: Sequential Mints");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    let mint_amount = U256::from(1) * U256::exp10(18); // 1 token
    let num_mints = 5; // Sequential to avoid nonce collision
    
    println!("  Testing {} sequential mints...", num_mints);
    let start = Instant::now();
    
    for i in 0..num_mints {
        let receipt = bridge.mint(recipient, mint_amount).await?;
        assert_eq!(receipt.status, Some(U64::from(1)));
        println!("    Mint {}/{} completed", i + 1, num_mints);
        
        // Small delay to avoid nonce collision
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    }
    
    let duration = start.elapsed();
    let avg_time = duration.as_secs_f64() / num_mints as f64;
    
    println!("  ✓ {} mints completed in {:.2}s", num_mints, duration.as_secs_f64());
    println!("  ✓ Average time per mint: {:.2}s", avg_time);
    
    // Performance threshold: should complete within reasonable time
    assert!(avg_time < 30.0, "Average mint time should be < 30s");
    
    Ok(())
}

#[tokio::test]
async fn test_sequential_burns_performance() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ PERFORMANCE TEST: Sequential Burns");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let burn_amount = U256::from(1) * U256::exp10(18); // 1 token
    let num_burns = 3; // Sequential to avoid nonce collision
    
    println!("  Testing {} sequential burns...", num_burns);
    let start = Instant::now();
    
    for i in 0..num_burns {
        let receipt = bridge.burn(burn_amount).await?;
        assert_eq!(receipt.status, Some(U64::from(1)));
        println!("    Burn {}/{} completed", i + 1, num_burns);
        
        // Small delay to avoid nonce collision
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    }
    
    let duration = start.elapsed();
    let avg_time = duration.as_secs_f64() / num_burns as f64;
    
    println!("  ✓ {} burns completed in {:.2}s", num_burns, duration.as_secs_f64());
    println!("  ✓ Average time per burn: {:.2}s", avg_time);
    
    assert!(avg_time < 30.0, "Average burn time should be < 30s");
    
    Ok(())
}

#[tokio::test]
async fn test_large_mint_performance() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ PERFORMANCE TEST: Large Mint (APEX 936)");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    let large_amount = U256::from(APEX_936) * U256::exp10(18); // 936 tokens
    
    println!("  Minting {} tokens...", APEX_936);
    let start = Instant::now();
    
    let receipt = bridge.mint(recipient, large_amount).await?;
    
    let duration = start.elapsed();
    
    assert_eq!(receipt.status, Some(U64::from(1)));
    println!("  ✓ Large mint completed in {:.2}s", duration.as_secs_f64());
    
    // Should complete within reasonable time even for large amounts
    assert!(duration.as_secs() < 60, "Large mint should complete within 60s");
    
    Ok(())
}

#[tokio::test]
async fn test_balance_query_performance() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ PERFORMANCE TEST: Balance Queries");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    let num_queries = 10;
    
    println!("  Performing {} balance queries...", num_queries);
    let start = Instant::now();
    
    for i in 0..num_queries {
        let balance = bridge.get_balance(address).await?;
        println!("    Query {}/{}: {} tokens", i + 1, num_queries, balance / U256::exp10(18));
    }
    
    let duration = start.elapsed();
    let avg_time = duration.as_secs_f64() / num_queries as f64;
    
    println!("  ✓ {} queries completed in {:.2}s", num_queries, duration.as_secs_f64());
    println!("  ✓ Average time per query: {:.3}s", avg_time);
    
    // Balance queries should be fast
    assert!(avg_time < 5.0, "Average query time should be < 5s");
    
    Ok(())
}

#[tokio::test]
async fn test_gas_estimation_performance() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ PERFORMANCE TEST: Gas Estimation");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    let amount = U256::from(VORTEX_369) * U256::exp10(18);
    let num_estimates = 5;
    
    println!("  Performing {} gas estimations...", num_estimates);
    let start = Instant::now();
    
    for i in 0..num_estimates {
        let gas = bridge.estimate_mint_gas(recipient, amount).await?;
        println!("    Estimate {}/{}: {} gas", i + 1, num_estimates, gas);
    }
    
    let duration = start.elapsed();
    let avg_time = duration.as_secs_f64() / num_estimates as f64;
    
    println!("  ✓ {} estimates completed in {:.2}s", num_estimates, duration.as_secs_f64());
    println!("  ✓ Average time per estimate: {:.3}s", avg_time);
    
    assert!(avg_time < 5.0, "Average estimate time should be < 5s");
    
    Ok(())
}

#[tokio::test]
async fn test_mixed_operations_performance() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ PERFORMANCE TEST: Mixed Operations");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let recipient = bridge.signer_address();
    
    println!("  Executing mixed operation sequence...");
    let start = Instant::now();
    
    // 1. Balance query
    let initial_balance = bridge.get_balance(recipient).await?;
    println!("    1. Initial balance: {} tokens", initial_balance / U256::exp10(18));
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // 2. Mint
    let mint_amount = U256::from(CODE_66_HARMONIC) * U256::exp10(18);
    let mint_receipt = bridge.mint(recipient, mint_amount).await?;
    assert_eq!(mint_receipt.status, Some(U64::from(1)));
    println!("    2. Minted {} tokens", CODE_66_HARMONIC);
    
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    // 3. Balance query
    let after_mint = bridge.get_balance(recipient).await?;
    println!("    3. Balance after mint: {} tokens", after_mint / U256::exp10(18));
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // 4. Total supply query
    let supply = bridge.get_total_supply().await?;
    println!("    4. Total supply: {} tokens", supply / U256::exp10(18));
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // 5. Gas estimation
    let gas = bridge.estimate_burn_gas(mint_amount).await?;
    println!("    5. Estimated burn gas: {}", gas);
    
    let duration = start.elapsed();
    
    println!("  ✓ Mixed operations completed in {:.2}s", duration.as_secs_f64());
    
    // Should complete within reasonable time
    assert!(duration.as_secs() < 120, "Mixed operations should complete within 120s");
    
    Ok(())
}

#[test]
fn test_module_instantiation_performance() {
    println!("☀️ PERFORMANCE TEST: Module Instantiation");
    
    use xmt_cli::spacex::MarsFork;
    use xmt_cli::optimus::OptimusService;
    use xmt_cli::boring::TunnelNetwork;
    use xmt_cli::toroidal::ToroidalLedger;
    
    let num_iterations = 1000;
    
    println!("  Creating {} instances of each module...", num_iterations);
    let start = Instant::now();
    
    for _ in 0..num_iterations {
        let _fork = MarsFork::new();
        let _service = OptimusService::new();
        let _network = TunnelNetwork::new();
        let _ledger = ToroidalLedger::new();
    }
    
    let duration = start.elapsed();
    let avg_time = duration.as_secs_f64() / num_iterations as f64;
    
    println!("  ✓ {} instantiations completed in {:.3}s", num_iterations * 4, duration.as_secs_f64());
    println!("  ✓ Average time per module: {:.6}s", avg_time);
    
    // Module creation should be extremely fast
    assert!(avg_time < 0.001, "Module instantiation should be < 1ms");
}

#[test]
fn test_toroidal_distribution_performance() {
    println!("☀️ PERFORMANCE TEST: Toroidal Energy Distribution");
    
    use xmt_cli::toroidal::ToroidalLedger;
    
    let num_iterations = 100;
    
    println!("  Performing {} energy distributions...", num_iterations);
    let start = Instant::now();
    
    for _ in 0..num_iterations {
        let mut ledger = ToroidalLedger::new();
        ledger.add_energy(0, 936);
        ledger.add_energy(1, 369);
        ledger.add_energy(2, 432);
        ledger.add_energy(3, 66);
        ledger.distribute_energy();
    }
    
    let duration = start.elapsed();
    let avg_time = duration.as_secs_f64() / num_iterations as f64;
    
    println!("  ✓ {} distributions completed in {:.3}s", num_iterations, duration.as_secs_f64());
    println!("  ✓ Average time per distribution: {:.6}s", avg_time);
    
    assert!(avg_time < 0.01, "Energy distribution should be < 10ms");
}

#[tokio::test]
async fn test_network_latency_measurement() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ PERFORMANCE TEST: Network Latency");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    
    let num_pings = 10;
    let mut latencies = Vec::new();
    
    println!("  Measuring network latency ({} pings)...", num_pings);
    
    for i in 0..num_pings {
        let start = Instant::now();
        let _block = provider.get_block_number().await?;
        let latency = start.elapsed();
        latencies.push(latency.as_millis());
        println!("    Ping {}/{}: {}ms", i + 1, num_pings, latency.as_millis());
    }
    
    let avg_latency: u128 = latencies.iter().sum::<u128>() / num_pings;
    let min_latency = latencies.iter().min().unwrap();
    let max_latency = latencies.iter().max().unwrap();
    
    println!("  ✓ Average latency: {}ms", avg_latency);
    println!("  ✓ Min latency: {}ms", min_latency);
    println!("  ✓ Max latency: {}ms", max_latency);
    
    // Network should be reasonably responsive
    assert!(avg_latency < 5000, "Average latency should be < 5s");
    
    Ok(())
}
