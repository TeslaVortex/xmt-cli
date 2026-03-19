//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// AUTO-REGISTER MODULE
// Automatically register X posts as vectors on mainnet
// EN EEKE MAI EA ANOKAYI CHENAK ENLEA 🌞🔱❤️‍🔥♾️👑🪞
//

use anyhow::Result;
use colored::Colorize;
use ethers::prelude::*;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::contracts::vector_registry::{VectorRegistry, hash_vector_to_bytes32};
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;
use crate::xapi::XApiClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoRegisterConfig {
    pub auto_post: bool,
    pub auto_register: bool,
    pub network: String, // "mainnet" or "sepolia"
    pub dimensions: u64,
}

impl Default for AutoRegisterConfig {
    fn default() -> Self {
        Self {
            auto_post: true,
            auto_register: true,
            network: "sepolia".to_string(),
            dimensions: 384,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostVectorResult {
    pub tweet_id: String,
    pub tweet_text: String,
    pub vector_hash: String,
    pub tx_hash: Option<String>,
    pub block_number: Option<u64>,
    pub registered: bool,
    pub timestamp: String,
}

/// Auto-post to X and register as vector on mainnet
pub async fn auto_post_and_register(
    text: String,
    config: AutoRegisterConfig,
) -> Result<PostVectorResult> {
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!("{}", "   ⚡ AUTO-POST & REGISTER INITIATED ⚡".bright_yellow().bold());
    println!("{}", "   X POST → MAINNET VECTOR REGISTRATION".bright_cyan().bold());
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!();

    let mut result = PostVectorResult {
        tweet_id: String::new(),
        tweet_text: text.clone(),
        vector_hash: String::new(),
        tx_hash: None,
        block_number: None,
        registered: false,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    // Step 1: Post to X if enabled
    if config.auto_post {
        println!("{}", "📡 STEP 1: POSTING TO X API...".bright_cyan().bold());
        println!();
        
        let client = XApiClient::from_env()?;
        let response = client.create_tweet(text.clone()).await?;
        
        result.tweet_id = response.data.id.clone();
        
        println!("{}", "✅ TWEET POSTED SUCCESSFULLY".bright_green().bold());
        println!("   {} {}", "Tweet ID:".bright_white(), response.data.id.bright_cyan());
        println!("   {} {}", "Text:".bright_white(), response.data.text.bright_white());
        println!();
    } else {
        println!("{}", "⏭️  STEP 1: SKIPPED (auto_post disabled)".yellow());
        println!();
    }

    // Step 2: Register as vector on-chain if enabled
    if config.auto_register {
        println!("{}", "⛓️  STEP 2: REGISTERING VECTOR ON-CHAIN...".bright_cyan().bold());
        println!();
        
        // Generate vector hash from text
        let vector_data = generate_384d_vector(&text);
        let vector_hash = hash_vector_to_bytes32(&vector_data);
        let vector_hash_hex = format!("0x{}", hex::encode(vector_hash));
        
        result.vector_hash = vector_hash_hex.clone();
        
        println!("   {} {}", "Vector Hash:".bright_white(), vector_hash_hex.bright_yellow());
        println!("   {} {}", "Dimensions:".bright_white(), config.dimensions.to_string().bright_cyan());
        println!("   {} {}", "Network:".bright_white(), config.network.bright_magenta());
        println!();
        
        // Get RPC and contract addresses based on network
        let (rpc_url, chain_id, registry_address) = get_network_config(&config.network)?;
        
        println!("   {} {}", "Chain ID:".bright_white(), chain_id.to_string().bright_cyan());
        println!("   {} {:?}", "Registry:".bright_white(), registry_address);
        println!();
        
        // Connect to chain
        let private_key = std::env::var("PRIVATE_KEY")?;
        let provider = Web3Provider::new(&rpc_url, chain_id).await?;
        let signer = WalletSigner::new(&private_key, chain_id)?;
        let client = Arc::new(signer.with_provider(provider.provider()));
        
        let registry = VectorRegistry::new(registry_address, client);
        
        println!("   {} Submitting transaction...", "⏳".yellow());
        
        // Register vector on-chain
        match registry.register_vector(vector_hash, &text, config.dimensions).await {
            Ok(receipt) => {
                result.tx_hash = Some(format!("{:?}", receipt.transaction_hash));
                result.block_number = receipt.block_number.map(|b| b.as_u64());
                result.registered = true;
                
                println!();
                println!("{}", "✅ VECTOR REGISTERED ON-CHAIN".bright_green().bold());
                println!("   {} {:?}", "Tx Hash:".bright_white(), receipt.transaction_hash);
                println!("   {} {:?}", "Block:".bright_white(), receipt.block_number);
                println!("   {} {:?}", "Gas Used:".bright_white(), receipt.gas_used);
                println!();
            }
            Err(e) => {
                println!();
                println!("{}", "⚠️  VECTOR REGISTRATION FAILED".yellow().bold());
                println!("   {} {}", "Error:".bright_red(), e.to_string());
                println!();
                println!("   Tweet was posted but vector not registered on-chain");
                println!();
            }
        }
    } else {
        println!("{}", "⏭️  STEP 2: SKIPPED (auto_register disabled)".yellow());
        println!();
    }

    // Display final summary
    display_summary(&result);

    Ok(result)
}

/// Register an existing tweet as a vector
pub async fn register_existing_tweet(
    tweet_id: String,
    tweet_text: String,
    network: String,
) -> Result<PostVectorResult> {
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!("{}", "   ⚡ REGISTER EXISTING TWEET AS VECTOR ⚡".bright_yellow().bold());
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!();

    println!("{} {}", "Tweet ID:".bright_magenta().bold(), tweet_id.bright_cyan());
    println!("{} {}", "Network:".bright_magenta().bold(), network.bright_cyan());
    println!();

    let mut result = PostVectorResult {
        tweet_id: tweet_id.clone(),
        tweet_text: tweet_text.clone(),
        vector_hash: String::new(),
        tx_hash: None,
        block_number: None,
        registered: false,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    // Generate vector hash
    let vector_data = generate_384d_vector(&tweet_text);
    let vector_hash = hash_vector_to_bytes32(&vector_data);
    let vector_hash_hex = format!("0x{}", hex::encode(vector_hash));
    
    result.vector_hash = vector_hash_hex.clone();
    
    println!("{} {}", "Vector Hash:".bright_white(), vector_hash_hex.bright_yellow());
    println!();

    // Get network config
    let (rpc_url, chain_id, registry_address) = get_network_config(&network)?;
    
    // Connect and register
    let private_key = std::env::var("PRIVATE_KEY")?;
    let provider = Web3Provider::new(&rpc_url, chain_id).await?;
    let signer = WalletSigner::new(&private_key, chain_id)?;
    let client = Arc::new(signer.with_provider(provider.provider()));
    
    let registry = VectorRegistry::new(registry_address, client);
    
    println!("{} Registering vector on-chain...", "⏳".yellow());
    
    let receipt = registry.register_vector(vector_hash, &tweet_text, 384).await?;
    
    result.tx_hash = Some(format!("{:?}", receipt.transaction_hash));
    result.block_number = receipt.block_number.map(|b| b.as_u64());
    result.registered = true;
    
    println!();
    println!("{}", "✅ VECTOR REGISTERED ON-CHAIN".bright_green().bold());
    println!("   {} {:?}", "Tx Hash:".bright_white(), receipt.transaction_hash);
    println!("   {} {:?}", "Block:".bright_white(), receipt.block_number);
    println!();

    display_summary(&result);

    Ok(result)
}

fn get_network_config(network: &str) -> Result<(String, u64, Address)> {
    match network {
        "mainnet" | "base" => {
            let rpc_url = std::env::var("BASE_RPC_URL")?;
            let chain_id = 8453;
            let registry_address: Address = std::env::var("VECTOR_REGISTRY_ADDRESS")?.parse()?;
            Ok((rpc_url, chain_id, registry_address))
        }
        "sepolia" => {
            let rpc_url = std::env::var("SEPOLIA_RPC_URL")?;
            let chain_id = 11155111;
            let registry_address: Address = std::env::var("VECTOR_REGISTRY_ADDRESS")?.parse()?;
            Ok((rpc_url, chain_id, registry_address))
        }
        _ => Err(anyhow::anyhow!("Unknown network: {}", network)),
    }
}

fn generate_384d_vector(text: &str) -> Vec<f32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let seed = hasher.finish();
    
    let mut vector = Vec::with_capacity(384);
    let mut rng_state = seed;
    
    for _ in 0..384 {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let normalized = (rng_state as f32) / (u64::MAX as f32);
        vector.push(normalized * 2.0 - 1.0);
    }
    
    vector
}

fn display_summary(result: &PostVectorResult) {
    println!("{}", "═══════════════════════════════════════════════════════".bright_yellow());
    println!("{}", "   ✅ AUTO-REGISTER COMPLETE".bright_green().bold());
    println!("{}", "═══════════════════════════════════════════════════════".bright_yellow());
    println!();
    
    if !result.tweet_id.is_empty() {
        println!("{} {}", "✓ Tweet Posted:".bright_green(), result.tweet_id.bright_cyan());
    }
    
    if !result.vector_hash.is_empty() {
        println!("{} {}", "✓ Vector Hash:".bright_green(), result.vector_hash.bright_yellow());
    }
    
    if result.registered {
        println!("{} {}", "✓ On-Chain:".bright_green(), "REGISTERED".bright_green().bold());
        if let Some(ref tx) = result.tx_hash {
            println!("  {} {}", "Tx:".bright_white(), tx.bright_cyan());
        }
        if let Some(block) = result.block_number {
            println!("  {} {}", "Block:".bright_white(), block.to_string().bright_cyan());
        }
    } else {
        println!("{} {}", "✓ On-Chain:".bright_yellow(), "NOT REGISTERED".bright_yellow());
    }
    
    println!();
    println!("{}", "EN EEKE MAI EA ANOKAYI CHENAK ENLEA 🌞🔱❤️‍🔥♾️👑🪞".bright_magenta().bold());
    println!("{}", "PAF PAF PAF — VECTOR SEALED ON-CHAIN".bright_yellow().bold());
    println!();
}
