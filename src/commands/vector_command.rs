//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Vector Commands - On-Chain Vector Operations
// Register, Mint, Stats for VectorRegistry + VectorMinter
// EN EEKE MAI EA ♾️♾️
//

use anyhow::Result;
use colored::Colorize;
use ethers::prelude::*;
use std::sync::Arc;

use crate::config::{APEX_936, VORTEX_369};
use crate::contracts::vector_registry::{VectorRegistry, hash_vector_to_bytes32};
use crate::contracts::vector_minter::VectorMinter;
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;

/// Main vector command dispatcher
pub fn vector_command(subcommand: &str, args: Vec<String>) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    
    match subcommand {
        "register" => {
            if args.is_empty() {
                println!("{}", "❌ Error: Intent required".red().bold());
                println!("Usage: xmt-cli vector register <intent> [--dimensions 384]");
                return;
            }
            let intent = args.join(" ");
            if let Err(e) = rt.block_on(vector_register(intent, 384)) {
                eprintln!("{}: {}", "Error".red().bold(), e);
            }
        }
        "mint" => {
            if args.is_empty() {
                println!("{}", "❌ Error: Vector hash required".red().bold());
                println!("Usage: xmt-cli vector mint <hash>");
                return;
            }
            let hash = args[0].clone();
            if let Err(e) = rt.block_on(vector_mint(hash)) {
                eprintln!("{}: {}", "Error".red().bold(), e);
            }
        }
        "stats" => {
            if args.is_empty() {
                if let Err(e) = rt.block_on(vector_total_stats()) {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                }
            } else {
                let hash = args[0].clone();
                if let Err(e) = rt.block_on(vector_hash_stats(hash)) {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                }
            }
        }
        "verify" => {
            if args.is_empty() {
                println!("{}", "❌ Error: Vector hash required".red().bold());
                println!("Usage: xmt-cli vector verify <hash>");
                return;
            }
            let hash = args[0].clone();
            if let Err(e) = rt.block_on(vector_verify(hash)) {
                eprintln!("{}: {}", "Error".red().bold(), e);
            }
        }
        _ => {
            print_help();
        }
    }
}

fn print_help() {
    println!("\n{}", "☀️☀️☀️ VECTOR COMMANDS ☀️☀️☀️".yellow().bold());
    println!("{}", "On-Chain Vector Registration & Minting".cyan());
    println!("{}", "EN EEKE MAI EA ♾️♾️\n".magenta());
    
    println!("{}", "USAGE:".green().bold());
    println!("  xmt-cli vector <SUBCOMMAND> [ARGS]\n");
    
    println!("{}", "SUBCOMMANDS:".green().bold());
    println!("  {}  Register intent as 384D vector on-chain", "register <intent>".cyan());
    println!("  {}  Mint XMT tokens for registered vector", "mint <hash>".cyan());
    println!("  {}  Get vector or total registry stats", "stats [hash]".cyan());
    println!("  {}  Verify vector exists on-chain", "verify <hash>".cyan());
    
    println!("\n{}", "EXAMPLES:".green().bold());
    println!("  xmt-cli vector register \"EN EEKE MAI EA 936\"");
    println!("  xmt-cli vector mint 0xabc123...");
    println!("  xmt-cli vector stats");
    println!("  xmt-cli vector verify 0xabc123...");
    
    println!("\n{}", "REQUIRED ENV VARS:".yellow().bold());
    println!("  SEPOLIA_RPC_URL           - Sepolia testnet RPC");
    println!("  PRIVATE_KEY               - Wallet private key");
    println!("  VECTOR_REGISTRY_ADDRESS   - VectorRegistry contract");
    println!("  VECTOR_MINTER_ADDRESS     - VectorMinter contract\n");
}

/// Register an intent as a vector on-chain
async fn vector_register(intent: String, dimensions: u64) -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "☀️☀️☀️ VECTOR REGISTER ☀️☀️☀️".yellow().bold());
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();
    
    // Get env vars - try Sepolia first, fall back to BASE
    let rpc_url = std::env::var("SEPOLIA_RPC_URL")
        .or_else(|_| std::env::var("BASE_RPC_URL"))?;
    let chain_id: u64 = std::env::var("SEPOLIA_CHAIN_ID")
        .unwrap_or_else(|_| "11155111".to_string())
        .parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let registry_address: Address = std::env::var("VECTOR_REGISTRY_ADDRESS")?.parse()?;
    
    println!("  Intent: \"{}\"", intent.cyan());
    println!("  Dimensions: {}", dimensions.to_string().yellow());
    println!("  Registry: {:?}", registry_address);
    println!();
    
    // Generate vector hash from intent
    let vector_data: Vec<f32> = generate_384d_vector(&intent);
    let vector_hash = hash_vector_to_bytes32(&vector_data);
    
    println!("  Vector Hash: 0x{}", hex::encode(vector_hash).yellow());
    println!();
    
    // Connect to chain
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let client = Arc::new(signer.with_provider(provider.provider()));
    
    let registry = VectorRegistry::new(registry_address, client);
    
    println!("  {} Registering vector on-chain...", "⏳".yellow());
    
    let receipt = registry.register_vector(vector_hash, &intent, dimensions).await?;
    
    println!();
    println!("{}", "✅ VECTOR REGISTERED ON-CHAIN".green().bold());
    println!("  Tx Hash: {:?}", receipt.transaction_hash);
    println!("  Block: {:?}", receipt.block_number);
    println!("  Gas Used: {:?}", receipt.gas_used);
    println!();
    println!("  {}", "Next: xmt-cli vector mint 0x...".cyan());
    println!();
    println!("{}", "EN EEKE MAI EA ♾️♾️".magenta().bold());
    
    Ok(())
}

/// Mint XMT tokens for a registered vector
async fn vector_mint(hash_str: String) -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "☀️☀️☀️ VECTOR MINT ☀️☀️☀️".yellow().bold());
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();
    
    let rpc_url = std::env::var("SEPOLIA_RPC_URL")
        .or_else(|_| std::env::var("BASE_RPC_URL"))?;
    let chain_id: u64 = std::env::var("SEPOLIA_CHAIN_ID")
        .unwrap_or_else(|_| "11155111".to_string())
        .parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let minter_address: Address = std::env::var("VECTOR_MINTER_ADDRESS")?.parse()?;
    
    // Parse hash
    let hash_clean = hash_str.trim_start_matches("0x");
    let hash_bytes = hex::decode(hash_clean)?;
    let mut vector_hash = [0u8; 32];
    vector_hash.copy_from_slice(&hash_bytes);
    
    println!("  Vector Hash: 0x{}", hash_clean.yellow());
    println!("  Minter: {:?}", minter_address);
    println!();
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let recipient = signer.address();
    let client = Arc::new(signer.with_provider(provider.provider()));
    
    let minter = VectorMinter::new(minter_address, client);
    
    // Check if minting is enabled
    let minting_enabled = minter.is_minting_enabled().await?;
    if !minting_enabled {
        println!("{}", "❌ Minting is currently disabled on VectorMinter".red().bold());
        return Ok(());
    }
    
    // Check if vector already minted
    let already_minted = minter.is_vector_minted(vector_hash).await?;
    if already_minted {
        println!("{}", "⚠️  Vector already minted".yellow().bold());
        let stats = minter.get_vector_stats(vector_hash).await?;
        println!("  Mint Amount: {} XMT", stats.mint_amount / U256::exp10(18));
        return Ok(());
    }
    
    // Calculate expected mint amount
    let expected_amount = minter.calculate_mint_amount(384).await?;
    println!("  Expected Mint: {} XMT", expected_amount / U256::exp10(18));
    println!("  Recipient: {:?}", recipient);
    println!();
    
    println!("  {} Minting tokens...", "⏳".yellow());
    
    let receipt = minter.mint_with_vector(vector_hash, recipient, None).await?;
    
    println!();
    println!("{}", "✅ TOKENS MINTED".green().bold());
    println!("  Tx Hash: {:?}", receipt.transaction_hash);
    println!("  Block: {:?}", receipt.block_number);
    println!("  Gas Used: {:?}", receipt.gas_used);
    println!();
    println!("{}", "ABUNDANCE FLOWS ♾️♾️".magenta().bold());
    
    Ok(())
}

/// Get total vector registry stats
async fn vector_total_stats() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "☀️☀️☀️ VECTOR STATS ☀️☀️☀️".yellow().bold());
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();
    
    let rpc_url = std::env::var("SEPOLIA_RPC_URL")
        .or_else(|_| std::env::var("BASE_RPC_URL"))?;
    let chain_id: u64 = std::env::var("SEPOLIA_CHAIN_ID")
        .unwrap_or_else(|_| "11155111".to_string())
        .parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let registry_address: Address = std::env::var("VECTOR_REGISTRY_ADDRESS")?.parse()?;
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let client = Arc::new(signer.with_provider(provider.provider()));
    
    let registry = VectorRegistry::new(registry_address, client);
    
    let total = registry.total_vectors().await?;
    let (apex, vortex, code, freq) = registry.get_sacred_numbers().await?;
    
    println!("  Registry: {:?}", registry_address);
    println!();
    println!("  {} Registered Vectors: {}", "📊".cyan(), total.to_string().yellow().bold());
    println!();
    println!("  Sacred Constants (On-Chain):");
    println!("    • APEX_936: {}", apex);
    println!("    • VORTEX_369: {}", vortex);
    println!("    • CODE_66: {}", code);
    println!("    • FREQUENCY_432: {}", freq);
    println!();
    println!("{}", "EN EEKE MAI EA ♾️♾️".magenta().bold());
    
    Ok(())
}

/// Get stats for a specific vector hash
async fn vector_hash_stats(hash_str: String) -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "☀️☀️☀️ VECTOR HASH STATS ☀️☀️☀️".yellow().bold());
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();
    
    let rpc_url = std::env::var("SEPOLIA_RPC_URL")
        .or_else(|_| std::env::var("BASE_RPC_URL"))?;
    let chain_id: u64 = std::env::var("SEPOLIA_CHAIN_ID")
        .unwrap_or_else(|_| "11155111".to_string())
        .parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let minter_address: Address = std::env::var("VECTOR_MINTER_ADDRESS")?.parse()?;
    
    let hash_clean = hash_str.trim_start_matches("0x");
    let hash_bytes = hex::decode(hash_clean)?;
    let mut vector_hash = [0u8; 32];
    vector_hash.copy_from_slice(&hash_bytes);
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let client = Arc::new(signer.with_provider(provider.provider()));
    
    let minter = VectorMinter::new(minter_address, client);
    let stats = minter.get_vector_stats(vector_hash).await?;
    
    println!("  Vector: 0x{}", hash_clean.yellow());
    println!();
    println!("  Minted: {}", if stats.minted { "✅ YES".green() } else { "❌ NO".red() });
    println!("  Mint Amount: {} XMT", stats.mint_amount / U256::exp10(18));
    println!("  Burn Amount: {} XMT", stats.burn_amount / U256::exp10(18));
    println!();
    println!("{}", "EN EEKE MAI EA ♾️♾️".magenta().bold());
    
    Ok(())
}

/// Verify a vector exists on-chain
async fn vector_verify(hash_str: String) -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "☀️☀️☀️ VECTOR VERIFY ☀️☀️☀️".yellow().bold());
    println!();
    
    let rpc_url = std::env::var("SEPOLIA_RPC_URL")
        .or_else(|_| std::env::var("BASE_RPC_URL"))?;
    let chain_id: u64 = std::env::var("SEPOLIA_CHAIN_ID")
        .unwrap_or_else(|_| "11155111".to_string())
        .parse()?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let registry_address: Address = std::env::var("VECTOR_REGISTRY_ADDRESS")?.parse()?;
    
    let hash_clean = hash_str.trim_start_matches("0x");
    let hash_bytes = hex::decode(hash_clean)?;
    let mut vector_hash = [0u8; 32];
    vector_hash.copy_from_slice(&hash_bytes);
    
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let client = Arc::new(signer.with_provider(provider.provider()));
    
    let registry = VectorRegistry::new(registry_address, client);
    let exists = registry.verify_vector(vector_hash).await?;
    
    println!("  Vector: 0x{}", hash_clean.yellow());
    println!("  Exists: {}", if exists { "✅ VERIFIED".green().bold() } else { "❌ NOT FOUND".red().bold() });
    println!();
    
    Ok(())
}

/// Generate a 384-dimensional vector from intent string
fn generate_384d_vector(intent: &str) -> Vec<f32> {
    use sha2::{Sha256, Digest};
    
    let mut vector = Vec::with_capacity(384);
    let mut hasher = Sha256::new();
    hasher.update(intent.as_bytes());
    
    // Generate 384 dimensions by iterating hash
    for i in 0u32..384 {
        let mut dim_hasher = Sha256::new();
        dim_hasher.update(hasher.clone().finalize());
        dim_hasher.update(i.to_le_bytes());
        let dim_hash = dim_hasher.finalize();
        
        // Convert first 4 bytes to f32 in range [-1, 1]
        let bytes: [u8; 4] = [dim_hash[0], dim_hash[1], dim_hash[2], dim_hash[3]];
        let raw = u32::from_le_bytes(bytes);
        let normalized = (raw as f64 / u32::MAX as f64) * 2.0 - 1.0;
        vector.push(normalized as f32);
    }
    
    // Apply sacred number modulation
    for (i, val) in vector.iter_mut().enumerate() {
        let modulation = match i % 9 {
            0 => 1.0 + (APEX_936 as f32 / 10000.0),
            3 | 6 => 1.0 + (VORTEX_369 as f32 / 10000.0),
            _ => 1.0,
        };
        *val *= modulation;
    }
    
    vector
}
