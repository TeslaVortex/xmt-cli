//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// SimulatedChain - THE KING WAITS FOR NOBODY
// Deterministic mock blockchain for offline/fallback operations
// EN EEKE MAI EA ♾️♾️
//

use anyhow::Result;
use ethers::types::{H256, Address, U256, U64};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorRecord {
    pub vector_hash: H256,
    pub intent: String,
    pub expanded_decree: Option<String>,
    pub creator: Address,
    pub timestamp: u64,
    pub dimensions: usize,
    pub block_number: U64,
    pub tx_hash: H256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedMint {
    pub tx_hash: H256,
    pub to: Address,
    pub amount: U256,
    pub vector_hash: H256,
    pub block_number: U64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedBurn {
    pub tx_hash: H256,
    pub from: Address,
    pub amount: U256,
    pub vector_hash: H256,
    pub block_number: U64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedReceipt {
    pub transaction_hash: H256,
    pub block_number: U64,
    pub gas_used: U256,
    pub status: bool,
    pub simulation_mode: bool,
}

pub struct SimulatedChain {
    vectors: Arc<Mutex<HashMap<H256, VectorRecord>>>,
    mint_events: Arc<Mutex<Vec<SimulatedMint>>>,
    burn_events: Arc<Mutex<Vec<SimulatedBurn>>>,
    block_number: Arc<Mutex<u64>>,
    default_address: Address,
}

impl SimulatedChain {
    pub fn new() -> Self {
        let default_address = "0x369369369369369369369369369369369369369"
            .parse()
            .unwrap_or_else(|_| Address::zero());
        
        Self {
            vectors: Arc::new(Mutex::new(HashMap::new())),
            mint_events: Arc::new(Mutex::new(Vec::new())),
            burn_events: Arc::new(Mutex::new(Vec::new())),
            block_number: Arc::new(Mutex::new(936)), // Start at sacred block 936
            default_address,
        }
    }

    fn next_block(&self) -> U64 {
        let mut block = self.block_number.lock().unwrap();
        *block += 1;
        U64::from(*block)
    }

    fn generate_tx_hash(&self, data: &[u8]) -> H256 {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0).to_le_bytes());
        let result = hasher.finalize();
        H256::from_slice(&result)
    }

    pub fn hash_vector(vector: &[f32]) -> H256 {
        let mut hasher = Sha256::new();
        for val in vector {
            hasher.update(val.to_le_bytes());
        }
        let result = hasher.finalize();
        H256::from_slice(&result)
    }

    pub fn register_vector(
        &self,
        intent: &str,
        expanded_decree: Option<&str>,
        vector: &[f32],
        creator: Option<Address>,
    ) -> Result<SimulatedReceipt> {
        let vector_hash = Self::hash_vector(vector);
        let block_number = self.next_block();
        let tx_hash = self.generate_tx_hash(intent.as_bytes());
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let creator_addr = creator.unwrap_or(self.default_address);

        let record = VectorRecord {
            vector_hash,
            intent: intent.to_string(),
            expanded_decree: expanded_decree.map(|s| s.to_string()),
            creator: creator_addr,
            timestamp,
            dimensions: vector.len(),
            block_number,
            tx_hash,
        };

        self.vectors.lock().unwrap().insert(vector_hash, record);

        println!("🔮 [SIMULATED] Vector registered on-chain");
        println!("   Hash: {:?}", vector_hash);
        println!("   Block: {}", block_number);
        println!("   Tx: {:?}", tx_hash);

        Ok(SimulatedReceipt {
            transaction_hash: tx_hash,
            block_number,
            gas_used: U256::from(66_369), // Sacred gas amount
            status: true,
            simulation_mode: true,
        })
    }

    pub fn mint_with_vector(
        &self,
        to: Address,
        amount: U256,
        vector_hash: H256,
    ) -> Result<SimulatedReceipt> {
        let block_number = self.next_block();
        let tx_hash = self.generate_tx_hash(&amount.as_u128().to_le_bytes());
        let timestamp = chrono::Utc::now().timestamp() as u64;

        let mint = SimulatedMint {
            tx_hash,
            to,
            amount,
            vector_hash,
            block_number,
            timestamp,
        };

        self.mint_events.lock().unwrap().push(mint);

        println!("⚡ [SIMULATED] Mint executed");
        println!("   To: {:?}", to);
        println!("   Amount: {} tokens", amount);
        println!("   Vector Hash: {:?}", vector_hash);
        println!("   Block: {}", block_number);

        Ok(SimulatedReceipt {
            transaction_hash: tx_hash,
            block_number,
            gas_used: U256::from(93_600), // Sacred gas
            status: true,
            simulation_mode: true,
        })
    }

    pub fn burn_with_vector(
        &self,
        from: Address,
        amount: U256,
        vector_hash: H256,
    ) -> Result<SimulatedReceipt> {
        let block_number = self.next_block();
        let tx_hash = self.generate_tx_hash(&amount.as_u128().to_le_bytes());
        let timestamp = chrono::Utc::now().timestamp() as u64;

        let burn = SimulatedBurn {
            tx_hash,
            from,
            amount,
            vector_hash,
            block_number,
            timestamp,
        };

        self.burn_events.lock().unwrap().push(burn);

        println!("🔥 [SIMULATED] Burn executed - PAF PAF PAF");
        println!("   From: {:?}", from);
        println!("   Amount: {} tokens obliterated", amount);
        println!("   Vector Hash: {:?}", vector_hash);
        println!("   Block: {}", block_number);

        Ok(SimulatedReceipt {
            transaction_hash: tx_hash,
            block_number,
            gas_used: U256::from(43_200), // 432 * 100 sacred gas
            status: true,
            simulation_mode: true,
        })
    }

    #[allow(dead_code)]
    pub fn get_vector(&self, hash: H256) -> Option<VectorRecord> {
        self.vectors.lock().unwrap().get(&hash).cloned()
    }

    #[allow(dead_code)]
    pub fn get_all_vectors(&self) -> Vec<VectorRecord> {
        self.vectors.lock().unwrap().values().cloned().collect()
    }

    #[allow(dead_code)]
    pub fn get_mint_events(&self) -> Vec<SimulatedMint> {
        self.mint_events.lock().unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn get_burn_events(&self) -> Vec<SimulatedBurn> {
        self.burn_events.lock().unwrap().clone()
    }

    pub fn current_block(&self) -> U64 {
        U64::from(*self.block_number.lock().unwrap())
    }

    #[allow(dead_code)]
    pub fn estimate_gas(&self, _operation: &str) -> U256 {
        // Return sacred gas estimates
        U256::from(66_369)
    }

    pub fn status(&self) -> String {
        let vectors = self.vectors.lock().unwrap().len();
        let mints = self.mint_events.lock().unwrap().len();
        let burns = self.burn_events.lock().unwrap().len();
        let block = self.current_block();

        format!(
            "SimulatedChain Status:\n  Block: {}\n  Vectors: {}\n  Mints: {}\n  Burns: {}\n  Mode: SOVEREIGN SIMULATION",
            block, vectors, mints, burns
        )
    }
}

impl Default for SimulatedChain {
    fn default() -> Self {
        Self::new()
    }
}

pub fn store_simulated_receipt(intent: &str, receipt: &SimulatedReceipt) -> Result<()> {
    use std::fs;
    use std::path::Path;

    let sim_dir = Path::new("Local_storage/.xmt-vectors/simulated");
    fs::create_dir_all(sim_dir)?;

    let filename = intent
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .replace(' ', "_")
        .to_lowercase();

    let filepath = sim_dir.join(format!("{}_receipt.json", filename));

    let data = serde_json::json!({
        "intent": intent,
        "transaction_hash": format!("{:?}", receipt.transaction_hash),
        "block_number": receipt.block_number.as_u64(),
        "gas_used": receipt.gas_used.as_u64(),
        "status": receipt.status,
        "simulation_mode": receipt.simulation_mode,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "forge": "EN EEKE MAI EA",
        "note": "THE KING SIMULATES AND WAITS FOR NOBODY"
    });

    fs::write(&filepath, serde_json::to_string_pretty(&data)?)?;

    println!("💾 Simulated receipt stored: {}", filepath.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulated_chain_creation() {
        let chain = SimulatedChain::new();
        assert_eq!(chain.current_block(), U64::from(936));
    }

    #[test]
    fn test_vector_registration() {
        let chain = SimulatedChain::new();
        let vector: Vec<f32> = (0..384).map(|i| i as f32 * 0.01).collect();
        
        let receipt = chain.register_vector("TEST INTENT", None, &vector, None);
        assert!(receipt.is_ok());
        assert!(receipt.unwrap().status);
    }

    #[test]
    fn test_vector_hash_consistency() {
        let vector: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4];
        let hash1 = SimulatedChain::hash_vector(&vector);
        let hash2 = SimulatedChain::hash_vector(&vector);
        assert_eq!(hash1, hash2);
    }
}
