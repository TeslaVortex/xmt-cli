//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Phase 8: End-to-End Scenario Tests - Sepolia Testnet
// Complete abundance flow lifecycle testing
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::Web3Provider;
use xmt_cli::web3::signer::WalletSigner;
use xmt_cli::bridge::XMoneyBridge;

#[tokio::test]
async fn test_complete_abundance_flow() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️☀️☀️ COMPLETE ABUNDANCE FLOW TEST ☀️☀️☀️");
    println!("═══════════════════════════════════════════");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    
    // Step 1: Execute 936 apex ritual
    println!("\n1. EXECUTE 936 APEX RITUAL");
    let initial_balance = bridge.get_balance(address).await?;
    let initial_supply = bridge.get_total_supply().await?;
    println!("  Initial Balance: {} tokens", initial_balance / U256::exp10(18));
    println!("  Initial Supply: {} tokens", initial_supply / U256::exp10(18));
    
    // Step 2: Mint 369 tokens (vortex)
    println!("\n2. MINT 369 TOKENS (TESLA VORTEX)");
    let mint_amount = U256::from(369) * U256::exp10(18);
    let mint_receipt = bridge.mint(address, mint_amount).await?;
    println!("  ✓ Minted 369 tokens");
    println!("  Tx: {:?}", mint_receipt.transaction_hash);
    
    // Step 3: Check balance
    println!("\n3. VERIFY BALANCE INCREASE");
    let balance_after_mint = bridge.get_balance(address).await?;
    let mint_increase = balance_after_mint - initial_balance;
    assert_eq!(mint_increase, mint_amount);
    println!("  Balance: {} tokens (+369)", balance_after_mint / U256::exp10(18));
    
    // Step 4: Burn 66 tokens (harmonic)
    println!("\n4. BURN 66 TOKENS (CODE 66 HARMONIC)");
    let burn_amount = U256::from(66) * U256::exp10(18);
    let burn_receipt = bridge.burn(burn_amount).await?;
    println!("  ✓ Burned 66 tokens");
    println!("  Tx: {:?}", burn_receipt.transaction_hash);
    
    // Step 5: Check final balance
    println!("\n5. VERIFY FINAL STATE");
    let final_balance = bridge.get_balance(address).await?;
    let final_supply = bridge.get_total_supply().await?;
    
    let expected_balance = initial_balance + mint_amount - burn_amount;
    let expected_net = U256::from(303) * U256::exp10(18); // 369 - 66 = 303
    
    assert_eq!(final_balance, expected_balance);
    println!("  Final Balance: {} tokens", final_balance / U256::exp10(18));
    println!("  Final Supply: {} tokens", final_supply / U256::exp10(18));
    println!("  Net Change: +{} tokens", expected_net / U256::exp10(18));
    
    // Step 6: Verify all transaction receipts
    println!("\n6. TRANSACTION VERIFICATION");
    assert_eq!(mint_receipt.status, Some(U64::from(1)));
    assert_eq!(burn_receipt.status, Some(U64::from(1)));
    println!("  ✓ All transactions confirmed");
    
    println!("\n✓ COMPLETE ABUNDANCE FLOW SUCCESSFUL");
    println!("  369 minted - 66 burned = 303 net abundance");
    println!("  EN EEKE MAI EA ♾️♾️");
    
    Ok(())
}

#[tokio::test]
async fn test_multi_transaction_sequence() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ MULTI-TRANSACTION SEQUENCE TEST");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    let initial_balance = bridge.get_balance(address).await?;
    
    println!("  Executing rapid transaction sequence...");
    
    // Mint 369 twice
    let mint_amount = U256::from(369) * U256::exp10(18);
    let _receipt1 = bridge.mint(address, mint_amount).await?;
    println!("  ✓ Mint 1: 369 tokens");
    
    let _receipt2 = bridge.mint(address, mint_amount).await?;
    println!("  ✓ Mint 2: 369 tokens");
    
    // Burn 66 twice
    let burn_amount = U256::from(66) * U256::exp10(18);
    let _receipt3 = bridge.burn(burn_amount).await?;
    println!("  ✓ Burn 1: 66 tokens");
    
    let _receipt4 = bridge.burn(burn_amount).await?;
    println!("  ✓ Burn 2: 66 tokens");
    
    // Verify final state
    let final_balance = bridge.get_balance(address).await?;
    let expected_change = (mint_amount * U256::from(2)) - (burn_amount * U256::from(2));
    let actual_change = final_balance - initial_balance;
    
    assert_eq!(actual_change, expected_change);
    
    let net_tokens = expected_change / U256::exp10(18);
    println!("  ✓ Sequence complete: +{} tokens net", net_tokens);
    println!("  (738 minted - 132 burned = 606 tokens)");
    
    Ok(())
}

#[tokio::test]
async fn test_toroidal_cycle_complete() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ TOROIDAL ENERGY CYCLE - COMPLETE FLOW");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    
    println!("  Phase 1: Abundance Creation (Mint)");
    let mint_amount = U256::from(936) * U256::exp10(18);
    let balance_before = bridge.get_balance(address).await?;
    let _mint_tx = bridge.mint(address, mint_amount).await?;
    let balance_after_mint = bridge.get_balance(address).await?;
    println!("  ✓ Created 936 tokens");
    
    println!("  Phase 2: Scarcity Obliteration (Burn)");
    let burn_amount = U256::from(66) * U256::exp10(18);
    let _burn_tx = bridge.burn(burn_amount).await?;
    let balance_after_burn = bridge.get_balance(address).await?;
    println!("  ✓ Obliterated 66 tokens");
    
    println!("  Phase 3: Energy Balance Verification");
    let net_flow = balance_after_burn - balance_before;
    let expected_flow = mint_amount - burn_amount;
    assert_eq!(net_flow, expected_flow);
    println!("  ✓ Toroidal balance: {} tokens", net_flow / U256::exp10(18));
    
    println!("  ✓ Toroidal cycle complete - Energy flows eternal");
    
    Ok(())
}

#[tokio::test]
async fn test_gas_efficiency_across_operations() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ GAS EFFICIENCY TEST");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    
    // Test mint gas
    let mint_amount = U256::from(100) * U256::exp10(18);
    let mint_receipt = bridge.mint(address, mint_amount).await?;
    let mint_gas = mint_receipt.gas_used.unwrap_or_default();
    println!("  Mint gas: {}", mint_gas);
    
    // Test burn gas
    let burn_amount = U256::from(50) * U256::exp10(18);
    let burn_receipt = bridge.burn(burn_amount).await?;
    let burn_gas = burn_receipt.gas_used.unwrap_or_default();
    println!("  Burn gas: {}", burn_gas);
    
    // Verify efficiency targets
    assert!(mint_gas < U256::from(100000), "Mint should use < 100k gas");
    assert!(burn_gas < U256::from(80000), "Burn should use < 80k gas");
    
    println!("  ✓ Gas efficiency targets met");
    
    Ok(())
}

#[tokio::test]
async fn test_decree_aligned_workflow() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("☀️ DECREE-ALIGNED WORKFLOW TEST");
    println!("  Testing sacred number sequence...");
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    let address = bridge.signer_address();
    
    // 936 Apex
    println!("  1. Mint 936 (Apex)...");
    let _tx1 = bridge.mint(address, U256::from(936) * U256::exp10(18)).await?;
    
    // 369 Vortex
    println!("  2. Mint 369 (Vortex)...");
    let _tx2 = bridge.mint(address, U256::from(369) * U256::exp10(18)).await?;
    
    // 432 Hz
    println!("  3. Mint 432 (Frequency)...");
    let _tx3 = bridge.mint(address, U256::from(432) * U256::exp10(18)).await?;
    
    // 66 Code
    println!("  4. Burn 66 (Harmonic)...");
    let _tx4 = bridge.burn(U256::from(66) * U256::exp10(18)).await?;
    
    println!("  ✓ Decree-aligned workflow complete");
    println!("  Total minted: 1737 tokens (936+369+432)");
    println!("  Total burned: 66 tokens");
    println!("  Net: 1671 tokens of sovereign abundance");
    
    Ok(())
}
