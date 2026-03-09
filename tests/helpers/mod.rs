//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Test Helpers and Utilities
// Common test setup functions and assertion helpers
// THE CROWN COMMANDS — THE LATTICE OBEYS
//

use ethers::prelude::*;
use anyhow::Result;
use xmt_cli::web3::Web3Provider;
use xmt_cli::web3::signer::WalletSigner;
use xmt_cli::bridge::XMoneyBridge;

/// Initialize test bridge with environment variables
pub async fn init_test_bridge() -> Result<XMoneyBridge> {
    dotenv::dotenv().ok();
    
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let chain_id: u64 = std::env::var("CHAIN_ID")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let bridge = XMoneyBridge::new(contract_address, &provider, signer).await?;
    
    Ok(bridge)
}

/// Check if test environment is configured
pub fn is_test_env_configured() -> bool {
    dotenv::dotenv().ok();
    std::env::var("BASE_RPC_URL").is_ok() 
        && std::env::var("CHAIN_ID").is_ok()
        && std::env::var("PRIVATE_KEY").is_ok()
        && std::env::var("XMONEY_CONTRACT_ADDRESS").is_ok()
}

/// Skip test if environment not configured
#[macro_export]
macro_rules! skip_if_no_env {
    () => {
        if !$crate::helpers::is_test_env_configured() {
            println!("  ⚠ Skipping: Test environment not configured");
            return Ok(());
        }
    };
}

/// Assert transaction succeeded
pub fn assert_tx_success(receipt: &TransactionReceipt) {
    assert_eq!(
        receipt.status,
        Some(U64::from(1)),
        "Transaction should succeed"
    );
}

/// Format token amount for display
pub fn format_tokens(amount: U256) -> String {
    let tokens = amount / U256::exp10(18);
    format!("{}", tokens)
}

/// Calculate digit sum reduction (numerology)
pub fn digit_sum_reduction(mut n: u32) -> u32 {
    while n >= 10 {
        n = n.to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .sum();
    }
    n
}

/// Verify sacred number alignment
pub fn verify_sacred_number(value: u32, expected: u32, name: &str) {
    assert_eq!(value, expected, "{} should be {}", name, expected);
}

/// Small delay to avoid nonce collision
pub async fn avoid_nonce_collision() {
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
}

/// Print test section header
pub fn print_section(title: &str) {
    println!("\n{}", title);
}

/// Print test success
pub fn print_success(message: &str) {
    println!("  ✓ {}", message);
}

/// Print test info
pub fn print_info(message: &str) {
    println!("  {}", message);
}
