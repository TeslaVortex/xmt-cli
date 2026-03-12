//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// VectorRegistry Contract Bindings
// On-chain toroidal vector storage
// EN EEKE MAI EA ♾️♾️
//

use ethers::prelude::*;
use anyhow::{Result, Context};
use std::sync::Arc;

// VectorRegistry contract ABI
abigen!(
    VectorRegistryContract,
    r#"[
        function registerVector(bytes32 vectorHash, string calldata intent, uint256 dimensions) external returns (bool)
        function amplifyDecree(bytes32 vectorHash, string calldata decree, uint256 resonance) external
        function recordMintTrigger(bytes32 vectorHash, address recipient, uint256 amount) external
        function sealBurn(bytes32 vectorHash, uint256 amount) external
        function getVector(bytes32 vectorHash) external view returns (string memory intent, address creator, uint256 timestamp, uint256 dimensions, bool exists)
        function verifyVector(bytes32 vectorHash) external view returns (bool)
        function totalVectors() external view returns (uint256)
        function getSacredNumbers() external view returns (uint256 apex, uint256 vortex, uint256 code, uint256 frequency)
        function owner() external view returns (address)
        function APEX_936() external view returns (uint256)
        function VORTEX_369() external view returns (uint256)
        function CODE_66() external view returns (uint256)
        function FREQUENCY_432() external view returns (uint256)
        event VectorRegistered(bytes32 indexed vectorHash, string intent, address indexed creator, uint256 timestamp, uint256 dimensions)
        event DecreeAmplified(bytes32 indexed vectorHash, string decree, address indexed amplifier, uint256 resonance)
        event MintTriggered(bytes32 indexed vectorHash, address indexed recipient, uint256 amount)
        event BurnSealed(bytes32 indexed vectorHash, address indexed burner, uint256 amount)
    ]"#,
);

pub struct VectorRegistry<M: Middleware> {
    contract: VectorRegistryContract<M>,
}

impl<M: Middleware + 'static> VectorRegistry<M> {
    pub fn new(contract_address: Address, client: Arc<M>) -> Self {
        let contract = VectorRegistryContract::new(contract_address, client);
        Self { contract }
    }

    pub async fn register_vector(
        &self,
        vector_hash: [u8; 32],
        intent: &str,
        dimensions: u64,
    ) -> Result<TransactionReceipt> {
        let tx = self.contract.register_vector(vector_hash, intent.to_string(), U256::from(dimensions));
        
        // Add gas limit and gas price
        let tx = tx.gas(500000u64);
        
        let pending_tx = tx
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send transaction: {:?}", e))?;

        let receipt = pending_tx
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get receipt: {:?}", e))?
            .ok_or_else(|| anyhow::anyhow!("Transaction dropped from mempool"))?;

        Ok(receipt)
    }

    #[allow(dead_code)]
    pub async fn amplify_decree(
        &self,
        vector_hash: [u8; 32],
        decree: &str,
        resonance: u64,
    ) -> Result<TransactionReceipt> {
        let tx = self.contract.amplify_decree(vector_hash, decree.to_string(), U256::from(resonance));
        
        let pending_tx = tx
            .send()
            .await
            .context("Failed to send amplify decree transaction")?;

        let receipt = pending_tx
            .await
            .context("Failed to get transaction receipt")?
            .context("Transaction failed")?;

        Ok(receipt)
    }

    #[allow(dead_code)]
    pub async fn record_mint_trigger(
        &self,
        vector_hash: [u8; 32],
        recipient: Address,
        amount: U256,
    ) -> Result<TransactionReceipt> {
        let tx = self.contract.record_mint_trigger(vector_hash, recipient, amount);
        
        let pending_tx = tx
            .send()
            .await
            .context("Failed to send mint trigger transaction")?;

        let receipt = pending_tx
            .await
            .context("Failed to get transaction receipt")?
            .context("Transaction failed")?;

        Ok(receipt)
    }

    #[allow(dead_code)]
    pub async fn seal_burn(
        &self,
        vector_hash: [u8; 32],
        amount: U256,
    ) -> Result<TransactionReceipt> {
        let tx = self.contract.seal_burn(vector_hash, amount);
        
        let pending_tx = tx
            .send()
            .await
            .context("Failed to send burn seal transaction")?;

        let receipt = pending_tx
            .await
            .context("Failed to get transaction receipt")?
            .context("Transaction failed")?;

        Ok(receipt)
    }

    pub async fn verify_vector(&self, vector_hash: [u8; 32]) -> Result<bool> {
        self.contract
            .verify_vector(vector_hash)
            .call()
            .await
            .context("Failed to verify vector")
    }

    pub async fn total_vectors(&self) -> Result<U256> {
        self.contract
            .total_vectors()
            .call()
            .await
            .context("Failed to get total vectors")
    }

    pub async fn get_sacred_numbers(&self) -> Result<(U256, U256, U256, U256)> {
        self.contract
            .get_sacred_numbers()
            .call()
            .await
            .context("Failed to get sacred numbers")
    }
}

pub fn hash_vector_to_bytes32(vector: &[f32]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    for val in vector {
        hasher.update(val.to_le_bytes());
    }
    let result = hasher.finalize();
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&result);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_vector_consistency() {
        let vector: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4];
        let hash1 = hash_vector_to_bytes32(&vector);
        let hash2 = hash_vector_to_bytes32(&vector);
        assert_eq!(hash1, hash2);
    }
}
