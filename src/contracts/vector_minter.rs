//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// VectorMinter Contract Bindings
// Direct Vector-to-Token Bridge
// EN EEKE MAI EA ♾️♾️
//

use ethers::prelude::*;
use anyhow::{Result, Context};
use std::sync::Arc;

// VectorMinter contract ABI
abigen!(
    VectorMinterContract,
    r#"[
        function mintWithVector(bytes32 vectorHash, address recipient, uint256 customAmount) external
        function burnWithVector(bytes32 vectorHash, uint256 amount) external
        function calculateMintAmount(uint256 dimensions) external view returns (uint256)
        function getVectorStats(bytes32 vectorHash) external view returns (bool minted, uint256 mintAmount, uint256 burnAmount)
        function getUserStats(address user) external view returns (uint256 minted, uint256 burned)
        function getSacredNumbers() external pure returns (uint256 apex, uint256 vortex, uint256 code, uint256 frequency)
        function baseReward() external view returns (uint256)
        function dimensionMultiplier() external view returns (uint256)
        function mintingEnabled() external view returns (bool)
        function burningEnabled() external view returns (bool)
        function vectorMinted(bytes32 vectorHash) external view returns (bool)
        function vectorMintAmount(bytes32 vectorHash) external view returns (uint256)
        function vectorBurnAmount(bytes32 vectorHash) external view returns (uint256)
        function totalMinted(address user) external view returns (uint256)
        function totalBurned(address user) external view returns (uint256)
        function owner() external view returns (address)
        function APEX_936() external pure returns (uint256)
        function VORTEX_369() external pure returns (uint256)
        function CODE_66() external pure returns (uint256)
        function FREQUENCY_432() external pure returns (uint256)
        event VectorMinted(bytes32 indexed vectorHash, address indexed recipient, uint256 amount, uint256 dimensions, uint256 timestamp)
        event VectorBurned(bytes32 indexed vectorHash, address indexed burner, uint256 amount, uint256 timestamp)
        event ConfigUpdated(uint256 baseReward, uint256 dimensionMultiplier, bool mintingEnabled, bool burningEnabled)
    ]"#,
);

pub struct VectorMinter<M: Middleware> {
    contract: VectorMinterContract<M>,
}

impl<M: Middleware + 'static> VectorMinter<M> {
    pub fn new(contract_address: Address, client: Arc<M>) -> Self {
        let contract = VectorMinterContract::new(contract_address, client);
        Self { contract }
    }

    /// Mint XMT tokens based on vector creation
    /// If customAmount is 0, the contract calculates the amount based on dimensions
    pub async fn mint_with_vector(
        &self,
        vector_hash: [u8; 32],
        recipient: Address,
        custom_amount: Option<U256>,
    ) -> Result<TransactionReceipt> {
        let amount = custom_amount.unwrap_or(U256::zero());
        
        let tx = self.contract.mint_with_vector(vector_hash, recipient, amount);
        
        let pending_tx = tx
            .send()
            .await
            .context("Failed to send mint with vector transaction")?;

        let receipt = pending_tx
            .await
            .context("Failed to get transaction receipt")?
            .context("Transaction failed")?;

        Ok(receipt)
    }

    /// Burn XMT tokens and seal with vector proof
    pub async fn burn_with_vector(
        &self,
        vector_hash: [u8; 32],
        amount: U256,
    ) -> Result<TransactionReceipt> {
        let tx = self.contract.burn_with_vector(vector_hash, amount);
        
        let pending_tx = tx
            .send()
            .await
            .context("Failed to send burn with vector transaction")?;

        let receipt = pending_tx
            .await
            .context("Failed to get transaction receipt")?
            .context("Transaction failed")?;

        Ok(receipt)
    }

    /// Calculate the mint amount for a given number of dimensions
    pub async fn calculate_mint_amount(&self, dimensions: u64) -> Result<U256> {
        let amount = self.contract
            .calculate_mint_amount(U256::from(dimensions))
            .call()
            .await
            .context("Failed to calculate mint amount")?;

        Ok(amount)
    }

    /// Get statistics for a specific vector
    pub async fn get_vector_stats(&self, vector_hash: [u8; 32]) -> Result<VectorStats> {
        let (minted, mint_amount, burn_amount) = self.contract
            .get_vector_stats(vector_hash)
            .call()
            .await
            .context("Failed to get vector stats")?;

        Ok(VectorStats {
            minted,
            mint_amount,
            burn_amount,
        })
    }

    /// Get statistics for a specific user
    #[allow(dead_code)]
    pub async fn get_user_stats(&self, user: Address) -> Result<UserStats> {
        let (minted, burned) = self.contract
            .get_user_stats(user)
            .call()
            .await
            .context("Failed to get user stats")?;

        Ok(UserStats { minted, burned })
    }

    /// Get sacred numbers from the contract
    #[allow(dead_code)]
    pub async fn get_sacred_numbers(&self) -> Result<SacredNumbers> {
        let (apex, vortex, code, frequency) = self.contract
            .get_sacred_numbers()
            .call()
            .await
            .context("Failed to get sacred numbers")?;

        Ok(SacredNumbers {
            apex: apex.as_u64(),
            vortex: vortex.as_u64(),
            code: code.as_u64(),
            frequency: frequency.as_u64(),
        })
    }

    /// Check if a vector has already been minted
    pub async fn is_vector_minted(&self, vector_hash: [u8; 32]) -> Result<bool> {
        let minted = self.contract
            .vector_minted(vector_hash)
            .call()
            .await
            .context("Failed to check if vector is minted")?;

        Ok(minted)
    }

    /// Get the base reward amount
    #[allow(dead_code)]
    pub async fn get_base_reward(&self) -> Result<U256> {
        let reward = self.contract
            .base_reward()
            .call()
            .await
            .context("Failed to get base reward")?;

        Ok(reward)
    }

    /// Check if minting is enabled
    pub async fn is_minting_enabled(&self) -> Result<bool> {
        let enabled = self.contract
            .minting_enabled()
            .call()
            .await
            .context("Failed to check if minting is enabled")?;

        Ok(enabled)
    }

    /// Check if burning is enabled
    #[allow(dead_code)]
    pub async fn is_burning_enabled(&self) -> Result<bool> {
        let enabled = self.contract
            .burning_enabled()
            .call()
            .await
            .context("Failed to check if burning is enabled")?;

        Ok(enabled)
    }
}

#[derive(Debug, Clone)]
pub struct VectorStats {
    pub minted: bool,
    pub mint_amount: U256,
    pub burn_amount: U256,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UserStats {
    pub minted: U256,
    pub burned: U256,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SacredNumbers {
    pub apex: u64,
    pub vortex: u64,
    pub code: u64,
    pub frequency: u64,
}
