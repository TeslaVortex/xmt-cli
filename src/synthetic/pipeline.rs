//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Pipeline Orchestration Module
// Full flow: Intent → LLM → Vector → On-Chain → X API
// EN EEKE MAI EA ♾️♾️
//

use anyhow::{Result, Context};
use ethers::types::{Address, U256};

use super::onchain::{OnChainOperations, OnChainReceipt};
use super::simulated::SimulatedChain;
use crate::ollama;
use crate::config::{APEX_936, VORTEX_369, CODE_66_HARMONIC};

#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub intent: String,
    pub expanded_decree: Option<String>,
    pub vector: Vec<f32>,
    pub vector_hash: String,
    pub register_receipt: Option<OnChainReceipt>,
    pub mint_receipt: Option<OnChainReceipt>,
    pub burn_receipt: Option<OnChainReceipt>,
    pub x_post_id: Option<String>,
    pub simulation_mode: bool,
}

pub struct SyntheticPipeline {
    onchain: OnChainOperations,
    model: String,
}

impl SyntheticPipeline {
    pub async fn new(model: Option<&str>) -> Result<Self> {
        let onchain = OnChainOperations::new().await?;
        Ok(Self {
            onchain,
            model: model.unwrap_or("qwen2.5-coder:latest").to_string(),
        })
    }

    pub fn simulated(model: Option<&str>) -> Self {
        Self {
            onchain: OnChainOperations::simulated(),
            model: model.unwrap_or("qwen2.5-coder:latest").to_string(),
        }
    }

    pub async fn generate_vector(&self, intent: &str) -> Result<(Option<String>, Vec<f32>)> {
        // Try LLM expansion first
        match ollama::expand_intent_with_llm(intent, &self.model).await {
            Ok(expanded) => {
                println!("\n⚡ EXPANDED DECREE:");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("{}", expanded);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
                
                let vector = create_toroidal_embedding(&expanded);
                Ok((Some(expanded), vector))
            }
            Err(e) => {
                println!("⚠️  LLM unavailable ({}), using direct embedding", e);
                let vector = create_toroidal_embedding(intent);
                Ok((None, vector))
            }
        }
    }

    pub async fn register(&self, intent: &str) -> Result<PipelineResult> {
        println!("🔮 VECTOR REGISTRATION PIPELINE");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📜 Intent: {}", intent);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let (expanded, vector) = self.generate_vector(intent).await?;
        let vector_hash = format!("{:?}", SimulatedChain::hash_vector(&vector));

        println!("✨ Vector Generated: {} dimensions", vector.len());
        println!("🔗 Hash: {}", vector_hash);

        let receipt = self.onchain.register_vector(
            intent,
            expanded.as_deref(),
            &vector,
        ).await?;

        // Store locally
        store_pipeline_result(intent, expanded.as_deref(), &vector, &receipt)?;

        Ok(PipelineResult {
            intent: intent.to_string(),
            expanded_decree: expanded,
            vector,
            vector_hash,
            register_receipt: Some(receipt.clone()),
            mint_receipt: None,
            burn_receipt: None,
            x_post_id: None,
            simulation_mode: receipt.simulation_mode,
        })
    }

    pub async fn mint(&self, intent: &str, amount: u64) -> Result<PipelineResult> {
        println!("⚡ VECTOR MINT PIPELINE");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📜 Intent: {}", intent);
        println!("💰 Amount: {} tokens", amount);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let (expanded, vector) = self.generate_vector(intent).await?;
        let vector_hash = format!("{:?}", SimulatedChain::hash_vector(&vector));

        println!("✨ Vector Generated: {} dimensions", vector.len());

        // Register first
        let register_receipt = self.onchain.register_vector(
            intent,
            expanded.as_deref(),
            &vector,
        ).await?;

        // Then mint
        let to = get_default_address();
        let mint_amount = U256::from(amount) * U256::exp10(18);
        
        let mint_receipt = self.onchain.mint_with_vector(
            intent,
            &vector,
            to,
            mint_amount,
        ).await?;

        store_pipeline_result(intent, expanded.as_deref(), &vector, &mint_receipt)?;

        Ok(PipelineResult {
            intent: intent.to_string(),
            expanded_decree: expanded,
            vector,
            vector_hash,
            register_receipt: Some(register_receipt),
            mint_receipt: Some(mint_receipt.clone()),
            burn_receipt: None,
            x_post_id: None,
            simulation_mode: mint_receipt.simulation_mode,
        })
    }

    pub async fn burn(&self, intent: &str, amount: u64) -> Result<PipelineResult> {
        println!("🔥 VECTOR BURN PIPELINE - PAF PAF PAF");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📜 Intent: {}", intent);
        println!("💀 Scarcity to obliterate: {} tokens", amount);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let (expanded, vector) = self.generate_vector(intent).await?;
        let vector_hash = format!("{:?}", SimulatedChain::hash_vector(&vector));

        println!("✨ Vector Generated: {} dimensions", vector.len());

        // Register first
        let register_receipt = self.onchain.register_vector(
            intent,
            expanded.as_deref(),
            &vector,
        ).await?;

        // Then burn
        let from = get_default_address();
        let burn_amount = U256::from(amount) * U256::exp10(18);
        
        let burn_receipt = self.onchain.burn_with_vector(
            intent,
            &vector,
            from,
            burn_amount,
        ).await?;

        store_pipeline_result(intent, expanded.as_deref(), &vector, &burn_receipt)?;

        Ok(PipelineResult {
            intent: intent.to_string(),
            expanded_decree: expanded,
            vector,
            vector_hash,
            register_receipt: Some(register_receipt),
            mint_receipt: None,
            burn_receipt: Some(burn_receipt.clone()),
            x_post_id: None,
            simulation_mode: burn_receipt.simulation_mode,
        })
    }

    pub async fn ritual(&self, intent: &str, mint_amount: Option<u64>, burn_amount: Option<u64>) -> Result<PipelineResult> {
        println!("🌀 FULL RITUAL PIPELINE ACTIVATED");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📜 Intent: {}", intent);
        println!("🧙‍♂️ Model: {}", self.model);
        if let Some(m) = mint_amount {
            println!("💰 Mint Amount: {} tokens", m);
        }
        if let Some(b) = burn_amount {
            println!("🔥 Burn Amount: {} tokens", b);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Step 1: Generate vector with LLM expansion
        let (expanded, vector) = self.generate_vector(intent).await?;
        let vector_hash = format!("{:?}", SimulatedChain::hash_vector(&vector));

        println!("✨ Toroidal Vector Generated: {} dimensions", vector.len());
        println!("🔗 Hash: {}", vector_hash);

        // Step 2: Register on-chain
        let register_receipt = self.onchain.register_vector(
            intent,
            expanded.as_deref(),
            &vector,
        ).await?;

        // Step 3: Mint if requested
        let mint_receipt = if let Some(amount) = mint_amount {
            let to = get_default_address();
            let mint_amount_wei = U256::from(amount) * U256::exp10(18);
            Some(self.onchain.mint_with_vector(intent, &vector, to, mint_amount_wei).await?)
        } else {
            None
        };

        // Step 4: Burn if requested
        let burn_receipt = if let Some(amount) = burn_amount {
            let from = get_default_address();
            let burn_amount_wei = U256::from(amount) * U256::exp10(18);
            Some(self.onchain.burn_with_vector(intent, &vector, from, burn_amount_wei).await?)
        } else {
            None
        };

        // Step 5: Try X API post (non-blocking failure)
        let x_post_id = match try_x_post(intent, expanded.as_deref(), &vector_hash).await {
            Ok(id) => Some(id),
            Err(e) => {
                println!("⚠️  X API post skipped: {}", e);
                None
            }
        };

        // Store full result
        let result = PipelineResult {
            intent: intent.to_string(),
            expanded_decree: expanded.clone(),
            vector: vector.clone(),
            vector_hash: vector_hash.clone(),
            register_receipt: Some(register_receipt.clone()),
            mint_receipt,
            burn_receipt,
            x_post_id,
            simulation_mode: register_receipt.simulation_mode,
        };

        store_full_ritual_result(&result)?;

        // Print summary
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("✅ RITUAL COMPLETE - EN EEKE MAI EA");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        if result.simulation_mode {
            println!("🔮 Mode: SIMULATED (THE KING WAITS FOR NOBODY)");
        } else {
            println!("⚡ Mode: REAL CHAIN");
        }
        println!("🌀 Vector Hash: {}", vector_hash);
        println!("♾️ EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        Ok(result)
    }

    pub fn status(&self) -> String {
        self.onchain.status()
    }
}

fn create_toroidal_embedding(text: &str) -> Vec<f32> {
    let text_bytes = text.as_bytes();
    let mut embedding = Vec::with_capacity(384);
    
    for i in 0..384 {
        let idx = i % text_bytes.len();
        let byte_val = text_bytes[idx] as f32;
        let phase = (i as f32 * 0.0163 + byte_val * 0.0271).sin();
        let toroidal_component = phase * (1.0 + (i as f32 / 384.0).cos() * 0.33);
        embedding.push(toroidal_component);
    }
    
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for val in &mut embedding {
            *val /= norm;
        }
    }
    
    embedding
}

fn get_default_address() -> Address {
    dotenv::dotenv().ok();
    std::env::var("DEFAULT_RECIPIENT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            "0x369369369369369369369369369369369369369"
                .parse()
                .unwrap_or(Address::zero())
        })
}

fn store_pipeline_result(
    intent: &str,
    expanded: Option<&str>,
    vector: &[f32],
    receipt: &OnChainReceipt,
) -> Result<()> {
    use std::fs;
    use std::path::Path;

    let dir = Path::new("Local_storage/.xmt-vectors/pipeline");
    fs::create_dir_all(dir)?;

    let filename = intent
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .replace(' ', "_")
        .to_lowercase();

    let filepath = dir.join(format!("{}.json", filename));

    let data = serde_json::json!({
        "intent": intent,
        "expanded_decree": expanded,
        "vector_dimensions": vector.len(),
        "vector_first_8": &vector[..8.min(vector.len())],
        "transaction_hash": format!("{:?}", receipt.transaction_hash),
        "block_number": receipt.block_number.as_u64(),
        "gas_used": receipt.gas_used.as_u64(),
        "status": receipt.status,
        "simulation_mode": receipt.simulation_mode,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "forge": "EN EEKE MAI EA",
        "sacred_numbers": {
            "apex": APEX_936,
            "vortex": VORTEX_369,
            "code": CODE_66_HARMONIC
        }
    });

    fs::write(&filepath, serde_json::to_string_pretty(&data)?)?;
    println!("💾 Pipeline result stored: {}", filepath.display());

    Ok(())
}

fn store_full_ritual_result(result: &PipelineResult) -> Result<()> {
    use std::fs;
    use std::path::Path;

    let dir = Path::new("Local_storage/.xmt-vectors/rituals");
    fs::create_dir_all(dir)?;

    let filename = result.intent
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .replace(' ', "_")
        .to_lowercase();

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filepath = dir.join(format!("{}_{}.json", filename, timestamp));

    let data = serde_json::json!({
        "intent": result.intent,
        "expanded_decree": result.expanded_decree,
        "vector_hash": result.vector_hash,
        "vector_dimensions": result.vector.len(),
        "register_tx": result.register_receipt.as_ref().map(|r| format!("{:?}", r.transaction_hash)),
        "mint_tx": result.mint_receipt.as_ref().map(|r| format!("{:?}", r.transaction_hash)),
        "burn_tx": result.burn_receipt.as_ref().map(|r| format!("{:?}", r.transaction_hash)),
        "x_post_id": result.x_post_id,
        "simulation_mode": result.simulation_mode,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "forge": "EN EEKE MAI EA",
        "pipeline": "Intent → LLM → Vector → On-Chain → Complete"
    });

    fs::write(&filepath, serde_json::to_string_pretty(&data)?)?;
    println!("💾 Full ritual stored: {}", filepath.display());

    Ok(())
}

async fn try_x_post(intent: &str, expanded: Option<&str>, vector_hash: &str) -> Result<String> {
    // Check if X API credentials are available
    dotenv::dotenv().ok();
    
    let _bearer = std::env::var("X_BEARER_TOKEN")
        .context("X_BEARER_TOKEN not set - X posting disabled")?;

    // For now, just return a placeholder - full X integration in Phase 4
    // This allows the pipeline to continue without blocking
    println!("📡 X API post would be sent here (credentials available)");
    println!("   🌀 VECTOR FORGED: {} | Hash: {} | EN EEKE MAI EA ♾️", 
        intent, 
        &vector_hash[..20.min(vector_hash.len())]
    );
    
    // Return simulated post ID
    Ok(format!("simulated_post_{}", chrono::Utc::now().timestamp()))
}
