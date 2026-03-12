use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use anyhow::{Result, Context};
use std::sync::Arc;

pub mod signer;
pub mod retry;

pub struct Web3Provider {
    provider: Arc<Provider<Http>>,
    chain_id: u64,
}

impl Web3Provider {
    pub async fn new(rpc_url: &str, chain_id: u64) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)
            .context("Failed to create HTTP provider")?;
        
        let provider = Arc::new(provider);
        
        Ok(Self {
            provider,
            chain_id,
        })
    }

    pub fn provider(&self) -> Arc<Provider<Http>> {
        self.provider.clone()
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    pub async fn get_block_number(&self) -> Result<U64> {
        self.provider
            .get_block_number()
            .await
            .context("Failed to get block number")
    }

    pub async fn get_gas_price(&self) -> Result<U256> {
        self.provider
            .get_gas_price()
            .await
            .context("Failed to get gas price")
    }

    pub async fn estimate_gas(&self, tx: &TypedTransaction) -> Result<U256> {
        self.provider
            .estimate_gas(tx, None)
            .await
            .context("Failed to estimate gas")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_creation() {
        // This test requires a valid RPC URL
        // Skip in CI/CD environments
        if std::env::var("BASE_RPC_URL").is_ok() {
            let provider = Web3Provider::new("https://mainnet.base.org", 8453).await;
            assert!(provider.is_ok());
        }
    }
}
