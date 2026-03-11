//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// On-Chain Operations Module
// Real testnet + SimulatedChain fallback
// THE KING WAITS FOR NOBODY
// EN EEKE MAI EA ♾️♾️
//

use anyhow::{Result, Context};
use ethers::prelude::*;
use std::sync::Arc;

use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;
use crate::contracts::vector_registry::{VectorRegistry, hash_vector_to_bytes32};
use crate::bridge::XMoneyBridge;
use super::simulated::{SimulatedChain, SimulatedReceipt, store_simulated_receipt};

pub enum ChainMode {
    Real {
        rpc_url: String,
        chain_id: u64,
        private_key: String,
        registry_address: Address,
        xmoney_address: Address,
    },
    Simulated(SimulatedChain),
}

pub struct OnChainOperations {
    mode: ChainMode,
}

impl OnChainOperations {
    pub async fn new() -> Result<Self> {
        dotenv::dotenv().ok();

        // Try to connect to real chain
        match Self::try_real_connection().await {
            Ok(mode) => {
                println!("⚡ Connected to REAL blockchain");
                Ok(Self { mode })
            }
            Err(e) => {
                println!("⚠️  Real chain unavailable: {}", e);
                println!("🔮 Activating SIMULATED mode - THE KING WAITS FOR NOBODY");
                Ok(Self {
                    mode: ChainMode::Simulated(SimulatedChain::new()),
                })
            }
        }
    }

    pub fn simulated() -> Self {
        println!("🔮 Running in SIMULATED mode");
        Self {
            mode: ChainMode::Simulated(SimulatedChain::new()),
        }
    }

    async fn try_real_connection() -> Result<ChainMode> {
        let rpc_url = std::env::var("BASE_RPC_URL")
            .context("BASE_RPC_URL not set")?;
        let chain_id: u64 = std::env::var("CHAIN_ID")
            .context("CHAIN_ID not set")?
            .parse()
            .context("Invalid CHAIN_ID")?;
        let private_key = std::env::var("PRIVATE_KEY")
            .context("PRIVATE_KEY not set")?;
        let xmoney_address: Address = std::env::var("XMONEY_CONTRACT_ADDRESS")
            .context("XMONEY_CONTRACT_ADDRESS not set")?
            .parse()
            .context("Invalid XMONEY_CONTRACT_ADDRESS")?;
        
        // VectorRegistry address - use env var or default to zero (not deployed yet)
        let registry_address: Address = std::env::var("VECTOR_REGISTRY_ADDRESS")
            .unwrap_or_else(|_| "0x0000000000000000000000000000000000000000".to_string())
            .parse()
            .context("Invalid VECTOR_REGISTRY_ADDRESS")?;

        // Test connection
        let provider = Web3Provider::new(&rpc_url, chain_id).await?;
        let _block = provider.get_block_number().await?;
        
        // Verify private key is valid
        let _signer = WalletSigner::new(&private_key, chain_id)?;

        Ok(ChainMode::Real {
            rpc_url,
            chain_id,
            private_key,
            registry_address,
            xmoney_address,
        })
    }

    pub fn is_simulated(&self) -> bool {
        matches!(self.mode, ChainMode::Simulated(_))
    }

    pub async fn register_vector(
        &self,
        intent: &str,
        expanded_decree: Option<&str>,
        vector: &[f32],
    ) -> Result<OnChainReceipt> {
        match &self.mode {
            ChainMode::Real { rpc_url, chain_id, private_key, registry_address, .. } => {
                if *registry_address == Address::zero() {
                    // Registry not deployed, fall back to simulation
                    println!("⚠️  VectorRegistry not deployed, using simulation");
                    let sim = SimulatedChain::new();
                    let receipt = sim.register_vector(intent, expanded_decree, vector, None)?;
                    store_simulated_receipt(intent, &receipt)?;
                    return Ok(OnChainReceipt::from_simulated(receipt));
                }

                let vector_hash = hash_vector_to_bytes32(vector);
                let provider = Web3Provider::new(rpc_url, *chain_id).await?;
                let signer = WalletSigner::new(private_key, *chain_id)?;
                let client = Arc::new(signer.with_provider(provider.provider()));
                let registry = VectorRegistry::new(*registry_address, client);
                
                println!("📡 Registering vector on-chain...");
                let receipt = registry.register_vector(vector_hash, intent, vector.len() as u64).await?;
                
                println!("✅ Vector registered on REAL chain");
                println!("   Tx: {:?}", receipt.transaction_hash);
                println!("   Block: {:?}", receipt.block_number);

                Ok(OnChainReceipt {
                    transaction_hash: receipt.transaction_hash,
                    block_number: receipt.block_number.unwrap_or_default(),
                    gas_used: receipt.gas_used.unwrap_or_default(),
                    status: receipt.status.map(|s| s.as_u64() == 1).unwrap_or(false),
                    simulation_mode: false,
                })
            }
            ChainMode::Simulated(sim) => {
                let receipt = sim.register_vector(intent, expanded_decree, vector, None)?;
                store_simulated_receipt(intent, &receipt)?;
                Ok(OnChainReceipt::from_simulated(receipt))
            }
        }
    }

    pub async fn mint_with_vector(
        &self,
        intent: &str,
        vector: &[f32],
        to: Address,
        amount: U256,
    ) -> Result<OnChainReceipt> {
        let vector_hash = hash_vector_to_bytes32(vector);

        match &self.mode {
            ChainMode::Real { rpc_url, chain_id, private_key, xmoney_address, .. } => {
                let provider = Web3Provider::new(rpc_url, *chain_id).await?;
                let signer = WalletSigner::new(private_key, *chain_id)?;
                let bridge = XMoneyBridge::new(
                    *xmoney_address,
                    &provider,
                    signer,
                ).await?;

                println!("📡 Executing mint on-chain...");
                let receipt = bridge.mint(to, amount).await?;

                println!("✅ Mint executed on REAL chain");
                println!("   To: {:?}", to);
                println!("   Amount: {}", amount);
                println!("   Tx: {:?}", receipt.transaction_hash);

                Ok(OnChainReceipt {
                    transaction_hash: receipt.transaction_hash,
                    block_number: receipt.block_number.unwrap_or_default(),
                    gas_used: receipt.gas_used.unwrap_or_default(),
                    status: receipt.status.map(|s| s.as_u64() == 1).unwrap_or(false),
                    simulation_mode: false,
                })
            }
            ChainMode::Simulated(sim) => {
                let receipt = sim.mint_with_vector(to, amount, H256::from(vector_hash))?;
                store_simulated_receipt(intent, &receipt)?;
                Ok(OnChainReceipt::from_simulated(receipt))
            }
        }
    }

    pub async fn burn_with_vector(
        &self,
        intent: &str,
        vector: &[f32],
        from: Address,
        amount: U256,
    ) -> Result<OnChainReceipt> {
        let vector_hash = hash_vector_to_bytes32(vector);

        match &self.mode {
            ChainMode::Real { rpc_url, chain_id, private_key, xmoney_address, .. } => {
                let provider = Web3Provider::new(rpc_url, *chain_id).await?;
                let signer = WalletSigner::new(private_key, *chain_id)?;
                let bridge = XMoneyBridge::new(
                    *xmoney_address,
                    &provider,
                    signer,
                ).await?;

                println!("📡 Executing burn on-chain...");
                let receipt = bridge.burn(amount).await?;

                println!("✅ Burn executed on REAL chain - PAF PAF PAF");
                println!("   Amount: {} obliterated", amount);
                println!("   Tx: {:?}", receipt.transaction_hash);

                Ok(OnChainReceipt {
                    transaction_hash: receipt.transaction_hash,
                    block_number: receipt.block_number.unwrap_or_default(),
                    gas_used: receipt.gas_used.unwrap_or_default(),
                    status: receipt.status.map(|s| s.as_u64() == 1).unwrap_or(false),
                    simulation_mode: false,
                })
            }
            ChainMode::Simulated(sim) => {
                let receipt = sim.burn_with_vector(from, amount, H256::from(vector_hash))?;
                store_simulated_receipt(intent, &receipt)?;
                Ok(OnChainReceipt::from_simulated(receipt))
            }
        }
    }

    pub fn status(&self) -> String {
        match &self.mode {
            ChainMode::Real { chain_id, .. } => {
                format!("REAL Chain Mode\n  Chain ID: {}\n  Status: CONNECTED", chain_id)
            }
            ChainMode::Simulated(sim) => sim.status(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OnChainReceipt {
    pub transaction_hash: H256,
    pub block_number: U64,
    pub gas_used: U256,
    pub status: bool,
    pub simulation_mode: bool,
}

impl OnChainReceipt {
    pub fn from_simulated(receipt: SimulatedReceipt) -> Self {
        Self {
            transaction_hash: receipt.transaction_hash,
            block_number: receipt.block_number,
            gas_used: receipt.gas_used,
            status: receipt.status,
            simulation_mode: true,
        }
    }
}

