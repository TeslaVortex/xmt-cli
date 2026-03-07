//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Minter Role Verification Script
// Checks if wallet has minter permissions on X-Money contract
//

use ethers::prelude::*;
use anyhow::Result;
use colored::Colorize;
use std::sync::Arc;

abigen!(
    XMoneyContract,
    r#"[
        function hasRole(bytes32 role, address account) external view returns (bool)
        function MINTER_ROLE() external view returns (bytes32)
        function DEFAULT_ADMIN_ROLE() external view returns (bytes32)
        function getRoleAdmin(bytes32 role) external view returns (bytes32)
        function owner() external view returns (address)
    ]"#
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!("{}", "   MINTER ROLE VERIFICATION - SEPOLIA TESTNET".bright_yellow().bold());
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!();
    
    // Load configuration
    let rpc_url = std::env::var("BASE_RPC_URL")?;
    let contract_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")?.parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    
    // Create provider
    let provider = Provider::<Http>::try_from(&rpc_url)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    
    // Create wallet
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);
    let wallet_address = wallet.address();
    
    println!("{}", "1. Configuration".bright_blue().bold());
    println!("  Wallet: {:?}", wallet_address);
    println!("  Contract: {:?}", contract_address);
    println!("  Chain ID: {}", chain_id);
    println!();
    
    // Connect to contract
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));
    let contract = XMoneyContract::new(contract_address, client);
    
    println!("{}", "2. Checking Contract Roles...".bright_blue().bold());
    
    // Try to get MINTER_ROLE
    match contract.minter_role().call().await {
        Ok(minter_role) => {
            println!("{} {:?}", "  ✓ MINTER_ROLE:".bright_green(), minter_role);
            
            // Check if wallet has minter role
            match contract.has_role(minter_role, wallet_address).call().await {
                Ok(has_role) => {
                    if has_role {
                        println!("{}", "  ✓ Wallet HAS minter role".bright_green().bold());
                    } else {
                        println!("{}", "  ✗ Wallet DOES NOT have minter role".bright_red().bold());
                    }
                }
                Err(e) => {
                    println!("{} {}", "  ✗ Error checking role:".bright_red(), e);
                }
            }
            
            // Get role admin
            match contract.get_role_admin(minter_role).call().await {
                Ok(admin_role) => {
                    println!("{} {:?}", "  ✓ Role admin:".bright_green(), admin_role);
                }
                Err(e) => {
                    println!("{} {}", "  ⚠ Could not get role admin:".yellow(), e);
                }
            }
        }
        Err(e) => {
            println!("{} {}", "  ⚠ MINTER_ROLE not found (may use different access control):".yellow(), e);
        }
    }
    
    println!();
    
    // Try to get owner
    println!("{}", "3. Checking Contract Owner...".bright_blue().bold());
    match contract.owner().call().await {
        Ok(owner) => {
            println!("{} {:?}", "  ✓ Contract owner:".bright_green(), owner);
            if owner == wallet_address {
                println!("{}", "  ✓ Wallet IS the contract owner".bright_green().bold());
            } else {
                println!("{}", "  ✗ Wallet is NOT the contract owner".bright_red());
            }
        }
        Err(e) => {
            println!("{} {}", "  ⚠ Could not get owner (may not be Ownable):".yellow(), e);
        }
    }
    
    println!();
    println!("{}", "═══════════════════════════════════════".bright_yellow());
    println!("{}", "RECOMMENDATION:".bright_yellow().bold());
    println!();
    println!("If wallet does NOT have minter role, you need to:");
    println!("1. Connect to contract owner wallet");
    println!("2. Call grantRole(MINTER_ROLE, 0x62397a99e60d395702c4d8d4befccee7e01da491)");
    println!();
    println!("Or if using Ownable pattern:");
    println!("1. Ensure wallet is the owner");
    println!("2. Or call transferOwnership() from current owner");
    println!();
    println!("{}", "EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
    
    Ok(())
}
