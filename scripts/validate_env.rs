//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Environment Validation Script - Sepolia Testnet
// Verifies configuration before testing begins
//

use ethers::prelude::*;
use anyhow::{Result, bail};
use colored::Colorize;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!("{}", "   ENVIRONMENT VALIDATION - SEPOLIA TESTNET".bright_yellow().bold());
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!();
    
    let mut validation_passed = true;
    
    // 1. Validate RPC URL
    println!("{}", "1. Validating RPC URL...".bright_blue().bold());
    let rpc_url = match env::var("BASE_RPC_URL") {
        Ok(url) => {
            println!("{} {}", "  ✓ RPC URL found:".bright_green(), url.bright_cyan());
            url
        },
        Err(_) => {
            println!("{}", "  ✗ BASE_RPC_URL not found in .env".bright_red().bold());
            validation_passed = false;
            String::new()
        }
    };
    
    // 2. Validate Chain ID
    println!();
    println!("{}", "2. Validating Chain ID...".bright_blue().bold());
    let chain_id: u64 = match env::var("CHAIN_ID") {
        Ok(id) => match id.parse::<u64>() {
            Ok(parsed) => {
                println!("{} {}", "  ✓ Chain ID found:".bright_green(), parsed.to_string().bright_cyan());
                if parsed == 11155111 {
                    println!("{}", "  ✓ Ethereum Sepolia (11155111)".bright_green());
                } else if parsed == 84532 {
                    println!("{}", "  ✓ Base Sepolia (84532)".bright_green());
                } else if parsed == 8453 {
                    println!("{}", "  ⚠ Base Mainnet (8453) - WARNING: Using mainnet!".yellow().bold());
                } else {
                    println!("{} {}", "  ⚠ Unknown chain ID:".yellow(), parsed);
                }
                parsed
            },
            Err(_) => {
                println!("{}", "  ✗ Invalid CHAIN_ID format".bright_red().bold());
                validation_passed = false;
                0
            }
        },
        Err(_) => {
            println!("{}", "  ✗ CHAIN_ID not found in .env".bright_red().bold());
            validation_passed = false;
            0
        }
    };
    
    // 3. Validate Private Key
    println!();
    println!("{}", "3. Validating Private Key...".bright_blue().bold());
    let private_key = match env::var("PRIVATE_KEY") {
        Ok(key) => {
            if key.len() == 64 || key.starts_with("0x") && key.len() == 66 {
                println!("{}", "  ✓ Private key format valid".bright_green());
                
                // Try to parse as wallet
                match key.parse::<LocalWallet>() {
                    Ok(wallet) => {
                        let address = wallet.address();
                        println!("{} {}", "  ✓ Wallet address:".bright_green(), format!("{:?}", address).bright_cyan());
                        Some(key)
                    },
                    Err(e) => {
                        println!("{} {}", "  ✗ Failed to parse private key:".bright_red().bold(), e);
                        validation_passed = false;
                        None
                    }
                }
            } else {
                println!("{}", "  ✗ Invalid private key length".bright_red().bold());
                validation_passed = false;
                None
            }
        },
        Err(_) => {
            println!("{}", "  ✗ PRIVATE_KEY not found in .env".bright_red().bold());
            validation_passed = false;
            None
        }
    };
    
    // 4. Validate Contract Address
    println!();
    println!("{}", "4. Validating Contract Address...".bright_blue().bold());
    let contract_address: Option<Address> = match env::var("XMONEY_CONTRACT_ADDRESS") {
        Ok(addr) => {
            match addr.parse::<Address>() {
                Ok(parsed) => {
                    println!("{} {}", "  ✓ Contract address:".bright_green(), format!("{:?}", parsed).bright_cyan());
                    Some(parsed)
                },
                Err(_) => {
                    println!("{}", "  ✗ Invalid contract address format".bright_red().bold());
                    validation_passed = false;
                    None
                }
            }
        },
        Err(_) => {
            println!("{}", "  ✗ XMONEY_CONTRACT_ADDRESS not found in .env".bright_red().bold());
            validation_passed = false;
            None
        }
    };
    
    // 5. Test RPC Connection
    if !rpc_url.is_empty() && chain_id > 0 {
        println!();
        println!("{}", "5. Testing RPC Connection...".bright_blue().bold());
        
        match Provider::<Http>::try_from(&rpc_url) {
            Ok(provider) => {
                println!("{}", "  ✓ Provider created successfully".bright_green());
                
                // Get block number
                match provider.get_block_number().await {
                    Ok(block) => {
                        println!("{} {}", "  ✓ Current block number:".bright_green(), block.to_string().bright_cyan());
                    },
                    Err(e) => {
                        println!("{} {}", "  ✗ Failed to get block number:".bright_red().bold(), e);
                        validation_passed = false;
                    }
                }
                
                // Get chain ID from network
                match provider.get_chainid().await {
                    Ok(network_chain_id) => {
                        println!("{} {}", "  ✓ Network chain ID:".bright_green(), network_chain_id.to_string().bright_cyan());
                        
                        if network_chain_id.as_u64() != chain_id {
                            println!("{}", "  ✗ MISMATCH: .env chain ID doesn't match network!".bright_red().bold());
                            println!("{} {}", "    Expected:".yellow(), chain_id.to_string().bright_cyan());
                            println!("{} {}", "    Got:".yellow(), network_chain_id.to_string().bright_cyan());
                            validation_passed = false;
                        } else {
                            println!("{}", "  ✓ Chain ID matches network".bright_green());
                        }
                    },
                    Err(e) => {
                        println!("{} {}", "  ✗ Failed to get network chain ID:".bright_red().bold(), e);
                        validation_passed = false;
                    }
                }
                
                // Check wallet balance if we have private key
                if let Some(pk) = private_key {
                    if let Ok(wallet) = pk.parse::<LocalWallet>() {
                        let address = wallet.address();
                        match provider.get_balance(address, None).await {
                            Ok(balance) => {
                                let balance_eth = ethers::utils::format_ether(balance);
                                println!("{} {} ETH", "  ✓ Wallet balance:".bright_green(), balance_eth.bright_cyan());
                                
                                if balance.is_zero() {
                                    println!("{}", "  ⚠ WARNING: Zero balance - need testnet ETH for gas!".yellow().bold());
                                    validation_passed = false;
                                }
                            },
                            Err(e) => {
                                println!("{} {}", "  ✗ Failed to get balance:".bright_red().bold(), e);
                                validation_passed = false;
                            }
                        }
                    }
                }
                
                // Check contract exists
                if let Some(addr) = contract_address {
                    match provider.get_code(addr, None).await {
                        Ok(code) => {
                            if code.len() > 2 {
                                println!("{} {} bytes", "  ✓ Contract bytecode found:".bright_green(), code.len().to_string().bright_cyan());
                            } else {
                                println!("{}", "  ✗ No contract deployed at address!".bright_red().bold());
                                validation_passed = false;
                            }
                        },
                        Err(e) => {
                            println!("{} {}", "  ✗ Failed to get contract code:".bright_red().bold(), e);
                            validation_passed = false;
                        }
                    }
                }
            },
            Err(e) => {
                println!("{} {}", "  ✗ Failed to create provider:".bright_red().bold(), e);
                validation_passed = false;
            }
        }
    }
    
    // 6. Validate Sacred Configuration
    println!();
    println!("{}", "6. Validating Sacred Configuration...".bright_blue().bold());
    
    let apex = env::var("APEX_VALUE").unwrap_or_else(|_| "936".to_string());
    if apex == "936" {
        println!("{} {}", "  ✓ APEX_VALUE:".bright_green(), "936".bright_magenta().bold());
    } else {
        println!("{} {} (expected 936)", "  ⚠ APEX_VALUE:".yellow(), apex.bright_cyan());
    }
    
    let helios = env::var("HELIOS_SIGNATURE").unwrap_or_else(|_| "".to_string());
    if helios == "EN EEKE MAI EA" {
        println!("{} {}", "  ✓ HELIOS_SIGNATURE:".bright_green(), "EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
    } else {
        println!("{} {}", "  ⚠ HELIOS_SIGNATURE:".yellow(), helios.bright_cyan());
    }
    
    // Final Summary
    println!();
    println!("{}", "═══════════════════════════════════════".bright_blue());
    
    if validation_passed {
        println!("{}", "✓ VALIDATION SUCCESSFUL - 100% COHERENCE".bright_green().bold());
        println!("{}", "  Environment ready for Sepolia testing".bright_green());
        println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
        println!();
        Ok(())
    } else {
        println!("{}", "✗ VALIDATION FAILED - CONFIGURATION ERRORS DETECTED".bright_red().bold());
        println!("{}", "  Please fix the issues above before testing".bright_red());
        println!();
        bail!("Environment validation failed")
    }
}
