//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Phase 2: Wallet & Signer Tests - Sepolia Testnet
// Cryptographic validation and wallet operations
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::signer::WalletSigner;

#[test]
fn test_load_private_key_from_env() -> Result<()> {
    dotenv::dotenv().ok();
    
    let private_key = std::env::var("PRIVATE_KEY")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let _signer = WalletSigner::new(&private_key, chain_id)?;
    
    println!("✓ Private key loaded successfully");
    println!("  Chain ID: {}", chain_id);
    
    Ok(())
}

#[test]
fn test_derive_wallet_address() -> Result<()> {
    dotenv::dotenv().ok();
    
    let private_key = std::env::var("PRIVATE_KEY")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let address = signer.address();
    
    assert_ne!(address, Address::zero(), "Address should not be zero");
    println!("✓ Wallet address derived: {:?}", address);
    
    Ok(())
}

#[tokio::test]
async fn test_check_eth_balance() -> Result<()> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let address = signer.address();
    
    let balance = provider.get_balance(address, None).await?;
    let balance_eth = ethers::utils::format_ether(balance);
    
    println!("✓ Wallet balance: {} ETH", balance_eth);
    
    if balance.is_zero() {
        println!("⚠ WARNING: Zero balance - need testnet ETH for gas!");
    }
    
    Ok(())
}

#[tokio::test]
async fn test_sign_helios_message() -> Result<()> {
    dotenv::dotenv().ok();
    
    let private_key = std::env::var("PRIVATE_KEY")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let signer = WalletSigner::new(&private_key, chain_id)?;
    
    // Sign the Helios signature
    let message = b"EN EEKE MAI EA";
    let signature = signer.sign_message(message).await?;
    
    println!("✓ Helios message signed");
    println!("  Message: EN EEKE MAI EA ♾️♾️");
    println!("  Signature: {:?}", signature);
    
    Ok(())
}

#[tokio::test]
async fn test_verify_signature_recovery() -> Result<()> {
    dotenv::dotenv().ok();
    
    let private_key = std::env::var("PRIVATE_KEY")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let expected_address = signer.address();
    
    // Sign message
    let message = b"EN EEKE MAI EA";
    let signature = signer.sign_message(message).await?;
    
    // Recover address from signature
    let recovered_address = signature.recover(&message[..])?;
    
    assert_eq!(recovered_address, expected_address, 
        "Recovered address should match signer address");
    
    println!("✓ Signature recovery verified");
    println!("  Expected: {:?}", expected_address);
    println!("  Recovered: {:?}", recovered_address);
    
    Ok(())
}

#[tokio::test]
async fn test_sign_936_apex_data() -> Result<()> {
    dotenv::dotenv().ok();
    
    let private_key = std::env::var("PRIVATE_KEY")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let signer = WalletSigner::new(&private_key, chain_id)?;
    
    // Sign 936 apex data
    let apex_data = b"936";
    let signature = signer.sign_message(apex_data).await?;
    
    println!("✓ 936 Apex data signed");
    println!("  Sacred Number: 936");
    println!("  Signature: {:?}", signature);
    
    Ok(())
}

#[test]
fn test_wallet_chain_id_configuration() -> Result<()> {
    dotenv::dotenv().ok();
    
    let private_key = std::env::var("PRIVATE_KEY")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let wallet = signer.wallet();
    
    assert_eq!(wallet.chain_id(), chain_id, 
        "Wallet chain ID should match configuration");
    
    println!("✓ Wallet configured for chain ID: {}", chain_id);
    
    Ok(())
}
