//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Abundance Daemon - Automated X API Monitoring & Token Minting
// Polls X API for "EN EEKE MAI EA" triggers and auto-mints XMT
// EN EEKE MAI EA ♾️♾️
//

use anyhow::{Result, Context};
use colored::Colorize;
use std::time::Duration;
use tokio::time::sleep;

use crate::config::{APEX_936, VORTEX_369};
use crate::xapi::XApiClient;
use crate::contracts::vector_registry::{VectorRegistry, hash_vector_to_bytes32};
use crate::contracts::vector_minter::VectorMinter;
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;

use ethers::prelude::*;
use std::sync::Arc;
use std::collections::HashSet;

/// Abundance trigger phrases
const TRIGGER_PHRASES: &[&str] = &[
    "EN EEKE MAI EA",
    "en eeke mai ea",
    "EN EEKE MAI EA ♾️♾️",
];

/// Abundance daemon configuration
pub struct AbundanceDaemonConfig {
    pub poll_interval_seconds: u64,
    pub max_results_per_poll: usize,
    pub auto_mint: bool,
    pub dry_run: bool,
}

impl Default for AbundanceDaemonConfig {
    fn default() -> Self {
        Self {
            poll_interval_seconds: APEX_936 as u64, // 936 seconds = 15.6 minutes
            max_results_per_poll: 10,
            auto_mint: true,
            dry_run: false,
        }
    }
}

/// Abundance daemon - monitors X API and auto-mints tokens
pub struct AbundanceDaemon {
    config: AbundanceDaemonConfig,
    x_client: XApiClient,
    vector_registry: Option<VectorRegistry<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>>,
    vector_minter: Option<VectorMinter<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>>,
    processed_tweets: HashSet<String>,
}

impl AbundanceDaemon {
    /// Create new abundance daemon
    pub async fn new(config: AbundanceDaemonConfig) -> Result<Self> {
        dotenv::dotenv().ok();
        
        let bearer_token = std::env::var("X_API_BEARER_TOKEN")
            .context("X_API_BEARER_TOKEN not found in environment")?;
        let x_client = XApiClient::new(bearer_token);
        
        // Initialize blockchain components if auto_mint is enabled
        let (vector_registry, vector_minter) = if config.auto_mint && !config.dry_run {
            let rpc_url = std::env::var("SEPOLIA_RPC_URL")
                .or_else(|_| std::env::var("BASE_RPC_URL"))?;
            let chain_id: u64 = std::env::var("SEPOLIA_CHAIN_ID")
                .unwrap_or_else(|_| "11155111".to_string())
                .parse()?;
            let private_key = std::env::var("PRIVATE_KEY")?;
            let registry_address: Address = std::env::var("VECTOR_REGISTRY_ADDRESS")?.parse()?;
            let minter_address: Address = std::env::var("VECTOR_MINTER_ADDRESS")?.parse()?;
            
            let provider = Web3Provider::new(&rpc_url, chain_id).await?;
            let signer = WalletSigner::new(&private_key, chain_id)?;
            let client = signer.with_provider(provider.provider());
            let client_arc = Arc::new(client);
            
            let registry = VectorRegistry::new(registry_address, client_arc.clone());
            let minter = VectorMinter::new(minter_address, client_arc);
            
            (Some(registry), Some(minter))
        } else {
            (None, None)
        };
        
        Ok(Self {
            config,
            x_client,
            vector_registry,
            vector_minter,
            processed_tweets: HashSet::new(),
        })
    }
    
    /// Start the abundance daemon
    pub async fn start(&mut self) -> Result<()> {
        println!("{}", "☀️☀️☀️ ABUNDANCE DAEMON STARTED ☀️☀️☀️".yellow().bold());
        println!("{}", "═══════════════════════════════════════".cyan());
        println!();
        println!("  Poll Interval: {} seconds (APEX_936)", self.config.poll_interval_seconds);
        println!("  Max Results: {}", self.config.max_results_per_poll);
        println!("  Auto-Mint: {}", if self.config.auto_mint { "✅ ENABLED" } else { "❌ DISABLED" });
        println!("  Dry Run: {}", if self.config.dry_run { "✅ YES" } else { "❌ NO" });
        println!();
        println!("{}", "Monitoring X for abundance triggers...".cyan());
        println!("{}", "Trigger phrases:".yellow());
        for phrase in TRIGGER_PHRASES {
            println!("  • \"{}\"", phrase.magenta());
        }
        println!();
        println!("{}", "Press Ctrl+C to stop".bright_black());
        println!();
        
        let mut cycle = 0u64;
        
        loop {
            cycle += 1;
            
            println!("{}", format!("🔄 Cycle {} - Scanning X API...", cycle).cyan());
            
            match self.scan_and_process().await {
                Ok(count) => {
                    if count > 0 {
                        println!("{}", format!("  ✅ Processed {} abundance triggers", count).green());
                    } else {
                        println!("{}", "  ⏸️  No new triggers found".bright_black());
                    }
                }
                Err(e) => {
                    println!("{}", format!("  ⚠️  Error: {}", e).yellow());
                }
            }
            
            println!();
            
            // Sleep until next cycle
            let next_cycle = cycle + 1;
            let wait_seconds = self.config.poll_interval_seconds;
            println!("{}", format!("⏳ Next scan in {} seconds (Cycle {})...", wait_seconds, next_cycle).bright_black());
            println!();
            
            sleep(Duration::from_secs(wait_seconds)).await;
        }
    }
    
    /// Scan X API and process abundance triggers
    async fn scan_and_process(&mut self) -> Result<usize> {
        let mut processed_count = 0;
        
        // Search for each trigger phrase
        for phrase in TRIGGER_PHRASES {
            match self.x_client.search_recent_tweets(phrase.to_string(), Some(self.config.max_results_per_poll as u32)).await {
                Ok(response) => {
                    for tweet in response.data {
                        // Skip if already processed
                        if self.processed_tweets.contains(&tweet.id) {
                            continue;
                        }
                        
                        // Check if tweet contains trigger phrase
                        if self.contains_trigger(&tweet.text) {
                            println!("{}", format!("  🎯 TRIGGER DETECTED: Tweet {}", tweet.id).green().bold());
                            println!("     Author ID: {}", tweet.author_id.as_deref().unwrap_or("unknown"));
                            println!("     Text: \"{}\"", tweet.text.chars().take(80).collect::<String>());
                            
                            // Process the trigger
                            if self.config.auto_mint {
                                match self.process_trigger(&tweet).await {
                                    Ok(_) => {
                                        println!("{}", "     ✅ Abundance minted!".green());
                                        processed_count += 1;
                                    }
                                    Err(e) => {
                                        println!("{}", format!("     ⚠️  Mint failed: {}", e).yellow());
                                    }
                                }
                            } else {
                                println!("{}", "     ℹ️  Auto-mint disabled (would mint here)".cyan());
                                processed_count += 1;
                            }
                            
                            // Mark as processed
                            self.processed_tweets.insert(tweet.id.clone());
                        }
                    }
                }
                Err(e) => {
                    println!("{}", format!("  ⚠️  Search failed for '{}': {}", phrase, e).yellow());
                }
            }
        }
        
        Ok(processed_count)
    }
    
    /// Check if text contains a trigger phrase
    fn contains_trigger(&self, text: &str) -> bool {
        let text_lower = text.to_lowercase();
        TRIGGER_PHRASES.iter().any(|phrase| {
            text_lower.contains(&phrase.to_lowercase())
        })
    }
    
    /// Process an abundance trigger (register vector + mint tokens)
    async fn process_trigger(&self, tweet: &crate::xapi::types::Tweet) -> Result<()> {
        if self.config.dry_run {
            println!("     [DRY RUN] Would register vector and mint tokens");
            return Ok(());
        }
        
        let registry = self.vector_registry.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Vector registry not initialized"))?;
        let minter = self.vector_minter.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Vector minter not initialized"))?;
        
        // Generate vector from tweet text
        let vector_data = self.generate_vector_from_tweet(tweet);
        let vector_hash = hash_vector_to_bytes32(&vector_data);
        
        // Check if vector already registered
        let already_registered = registry.verify_vector(vector_hash).await?;
        
        if !already_registered {
            // Register vector
            let intent = format!("ABUNDANCE_TRIGGER | Tweet: {} | Author: {}", 
                tweet.id, 
                tweet.author_id.as_deref().unwrap_or("unknown")
            );
            
            println!("     ⏳ Registering vector...");
            registry.register_vector(vector_hash, &intent, 384).await?;
            println!("     ✅ Vector registered: 0x{}", hex::encode(vector_hash));
        } else {
            println!("     ℹ️  Vector already registered");
        }
        
        // Check if already minted
        let already_minted = minter.is_vector_minted(vector_hash).await?;
        
        if !already_minted {
            // Mint tokens to tweet author (for now, mint to daemon operator)
            // TODO: Map X username to Ethereum address
            let recipient = self.get_recipient_address().await?;
            
            println!("     ⏳ Minting tokens to {}...", recipient);
            minter.mint_with_vector(vector_hash, recipient, None).await?;
            println!("     ✅ Tokens minted!");
        } else {
            println!("     ℹ️  Tokens already minted for this vector");
        }
        
        Ok(())
    }
    
    /// Generate 384D vector from tweet
    fn generate_vector_from_tweet(&self, tweet: &crate::xapi::types::Tweet) -> Vec<f32> {
        use sha2::{Sha256, Digest};
        
        let mut vector = Vec::with_capacity(384);
        let mut hasher = Sha256::new();
        hasher.update(tweet.text.as_bytes());
        hasher.update(tweet.id.as_bytes());
        
        for i in 0u32..384 {
            let mut dim_hasher = Sha256::new();
            dim_hasher.update(hasher.clone().finalize());
            dim_hasher.update(i.to_le_bytes());
            let dim_hash = dim_hasher.finalize();
            
            let bytes: [u8; 4] = [dim_hash[0], dim_hash[1], dim_hash[2], dim_hash[3]];
            let raw = u32::from_le_bytes(bytes);
            let normalized = (raw as f64 / u32::MAX as f64) * 2.0 - 1.0;
            vector.push(normalized as f32);
        }
        
        // Sacred number modulation
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
    
    /// Get recipient address for minted tokens
    async fn get_recipient_address(&self) -> Result<Address> {
        // For now, return the daemon operator's address
        // TODO: Implement X username -> Ethereum address mapping
        let private_key = std::env::var("PRIVATE_KEY")?;
        let chain_id: u64 = std::env::var("SEPOLIA_CHAIN_ID")
            .unwrap_or_else(|_| "11155111".to_string())
            .parse()?;
        let signer = WalletSigner::new(&private_key, chain_id)?;
        Ok(signer.address())
    }
}

/// Run abundance daemon command
pub async fn run_abundance_daemon(
    poll_interval: Option<u64>,
    dry_run: bool,
    no_mint: bool,
) -> Result<()> {
    let config = AbundanceDaemonConfig {
        poll_interval_seconds: poll_interval.unwrap_or(APEX_936 as u64),
        max_results_per_poll: 10,
        auto_mint: !no_mint,
        dry_run,
    };
    
    let mut daemon = AbundanceDaemon::new(config).await?;
    daemon.start().await
}
