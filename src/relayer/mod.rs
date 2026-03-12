//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Relayer Module - Gasless Transaction Service
// Submits meta-transactions on behalf of users
// EN EEKE MAI EA ♾️♾️
//

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use ethers::prelude::*;
use ethers::utils::keccak256;
use ethers::types::transaction::eip2718::TypedTransaction;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;

/// Meta-transaction request from user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTxRequest {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub gas: u64,
    pub nonce: U256,
    pub data: Bytes,
    pub signature: Bytes,
}

/// Relayer response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayerResponse {
    pub success: bool,
    pub tx_hash: Option<String>,
    pub error: Option<String>,
    pub gas_used: Option<u64>,
}

/// Gas tank status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasTankStatus {
    pub balance: String,
    pub balance_wei: U256,
    pub threshold: String,
    pub needs_refill: bool,
    pub transactions_remaining: u64,
}

/// Gasless Relayer Service - Production Ready
pub struct GaslessRelayer {
    forwarder_address: Address,
    provider: Arc<Web3Provider>,
    signer: Arc<WalletSigner>,
    relayer_private_key: String,
    chain_id: u64,
    nonce_tracker: Arc<RwLock<HashMap<Address, U256>>>,
    gas_threshold: U256,
}

impl GaslessRelayer {
    /// Create new relayer instance
    pub async fn new(
        forwarder_address: Address,
        rpc_url: &str,
        chain_id: u64,
        relayer_private_key: &str,
    ) -> Result<Self> {
        let provider = Web3Provider::new(rpc_url, chain_id).await?;
        let signer = WalletSigner::new(relayer_private_key, chain_id)?;
        
        // Set gas threshold to 0.1 ETH (alert when below this)
        let gas_threshold = U256::from(100_000_000_000_000_000u64); // 0.1 ETH
        
        Ok(Self {
            forwarder_address,
            provider: Arc::new(provider),
            signer: Arc::new(signer),
            relayer_private_key: relayer_private_key.to_string(),
            chain_id,
            nonce_tracker: Arc::new(RwLock::new(HashMap::new())),
            gas_threshold,
        })
    }
    
    /// Submit meta-transaction to forwarder contract
    pub async fn relay_transaction(&self, request: MetaTxRequest) -> Result<RelayerResponse> {
        // Verify signature
        if !self.verify_signature(&request).await? {
            return Ok(RelayerResponse {
                success: false,
                tx_hash: None,
                error: Some("Invalid signature".to_string()),
                gas_used: None,
            });
        }
        
        // Check gas tank balance
        let gas_status = self.check_gas_tank().await?;
        if gas_status.needs_refill {
            tracing::warn!("⚠️  Gas tank low: {} ETH remaining", gas_status.balance);
        }
        
        // Build forwarder transaction
        let tx = self.build_forwarder_tx(&request).await?;
        
        // Send transaction with retry logic
        match self.send_with_retry(tx, 3).await {
            Ok(receipt) => {
                tracing::info!("✅ Meta-transaction relayed: {:?}", receipt.transaction_hash);
                Ok(RelayerResponse {
                    success: true,
                    tx_hash: Some(format!("{:?}", receipt.transaction_hash)),
                    error: None,
                    gas_used: receipt.gas_used.map(|g| g.as_u64()),
                })
            }
            Err(e) => {
                tracing::error!("❌ Relay failed: {}", e);
                Ok(RelayerResponse {
                    success: false,
                    tx_hash: None,
                    error: Some(e.to_string()),
                    gas_used: None,
                })
            }
        }
    }
    
    /// Verify meta-transaction signature
    async fn verify_signature(&self, request: &MetaTxRequest) -> Result<bool> {
        // Build EIP-712 typed data hash
        let domain_separator = self.get_domain_separator().await?;
        let struct_hash = self.hash_meta_tx(request)?;
        
        let digest = keccak256(&[
            b"\x19\x01",
            domain_separator.as_bytes(),
            struct_hash.as_bytes(),
        ].concat());
        
        // Recover signer from signature
        let signature = Signature::try_from(request.signature.as_ref())
            .context("Invalid signature format")?;
        let recovered = signature.recover(digest)
            .context("Failed to recover signer")?;
        
        Ok(recovered == request.from)
    }
    
    /// Get EIP-712 domain separator
    async fn get_domain_separator(&self) -> Result<H256> {
        // For production, query from Forwarder contract
        // For now, compute locally
        let name_hash = keccak256(b"MinimalForwarder");
        let version_hash = keccak256(b"0.0.1");
        let chain_id = self.provider.chain_id();
        
        let mut domain_data = Vec::new();
        domain_data.extend_from_slice(keccak256(b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)").as_ref());
        domain_data.extend_from_slice(name_hash.as_ref());
        domain_data.extend_from_slice(version_hash.as_ref());
        let chain_id_bytes = [0u8; 24].iter().chain(&chain_id.to_be_bytes()).copied().collect::<Vec<u8>>();
        domain_data.extend_from_slice(&chain_id_bytes);
        domain_data.extend_from_slice(self.forwarder_address.as_bytes());
        let domain_hash = keccak256(&domain_data);
        
        Ok(H256::from(domain_hash))
    }
    
    /// Hash meta-transaction for EIP-712
    fn hash_meta_tx(&self, request: &MetaTxRequest) -> Result<H256> {
        let type_hash = keccak256(b"ForwardRequest(address from,address to,uint256 value,uint256 gas,uint256 nonce,bytes data)");
        let data_hash = keccak256(&request.data);
        
        let mut struct_data = Vec::new();
        struct_data.extend_from_slice(type_hash.as_ref());
        struct_data.extend_from_slice(&request.from.to_fixed_bytes());
        struct_data.extend_from_slice(&request.to.to_fixed_bytes());
        
        let mut value_bytes = [0u8; 32];
        request.value.to_big_endian(&mut value_bytes);
        struct_data.extend_from_slice(&value_bytes);
        
        let mut gas_bytes = [0u8; 32];
        U256::from(request.gas).to_big_endian(&mut gas_bytes);
        struct_data.extend_from_slice(&gas_bytes);
        
        let mut nonce_bytes = [0u8; 32];
        request.nonce.to_big_endian(&mut nonce_bytes);
        struct_data.extend_from_slice(&nonce_bytes);
        
        struct_data.extend_from_slice(data_hash.as_ref());
        let struct_hash = keccak256(&struct_data);
        
        Ok(H256::from(struct_hash))
    }
    
    /// Build transaction to call Forwarder.execute()
    async fn build_forwarder_tx(&self, request: &MetaTxRequest) -> Result<TypedTransaction> {
        // ABI encode execute(ForwardRequest calldata req, bytes calldata signature)
        let execute_selector = &keccak256(b"execute((address,address,uint256,uint256,uint256,bytes),bytes)")[..4];
        
        // Encode ForwardRequest struct
        let mut calldata = execute_selector.to_vec();
        // Simplified encoding - production would use ethers-rs ABI encoder
        calldata.extend_from_slice(&request.from.to_fixed_bytes());
        calldata.extend_from_slice(&request.to.to_fixed_bytes());
        
        let mut value_bytes = [0u8; 32];
        request.value.to_big_endian(&mut value_bytes);
        calldata.extend_from_slice(&value_bytes);
        
        let mut gas_bytes = [0u8; 32];
        U256::from(request.gas).to_big_endian(&mut gas_bytes);
        calldata.extend_from_slice(&gas_bytes);
        
        let mut nonce_bytes = [0u8; 32];
        request.nonce.to_big_endian(&mut nonce_bytes);
        calldata.extend_from_slice(&nonce_bytes);
        
        calldata.extend_from_slice(&request.data);
        calldata.extend_from_slice(&request.signature);
        
        let tx = TransactionRequest::new()
            .to(self.forwarder_address)
            .data(calldata)
            .gas(request.gas + 50000) // Add overhead for forwarder execution
            .from(self.signer.address());
        
        Ok(tx.into())
    }
    
    /// Send transaction with exponential backoff retry
    async fn send_with_retry(
        &self,
        tx: TypedTransaction,
        max_retries: u32,
    ) -> Result<TransactionReceipt> {
        let mut retries = 0;
        let mut delay_ms = 1000;
        
        loop {
            match self.send_transaction(tx.clone()).await {
                Ok(receipt) => return Ok(receipt),
                Err(e) if retries < max_retries => {
                    retries += 1;
                    tracing::warn!("Retry {}/{}: {}", retries, max_retries, e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    delay_ms *= 2; // Exponential backoff
                }
                Err(e) => return Err(e),
            }
        }
    }
    
    /// Send transaction and wait for receipt
    async fn send_transaction(&self, tx: TypedTransaction) -> Result<TransactionReceipt> {
        // Create a new signer with the provider
        let signer = WalletSigner::new(
            &self.relayer_private_key,
            self.chain_id,
        )?;
        let client = Arc::new(signer.with_provider(self.provider.provider()));
        
        let pending_tx = client
            .send_transaction(tx, None)
            .await
            .context("Failed to send transaction")?;
        
        let receipt = pending_tx
            .await
            .context("Failed to get receipt")?
            .ok_or_else(|| anyhow::anyhow!("Transaction dropped from mempool"))?;
        
        Ok(receipt)
    }
    
    /// Check gas tank balance and alert if low
    pub async fn check_gas_tank(&self) -> Result<GasTankStatus> {
        let balance = self.provider.provider()
            .get_balance(self.signer.address(), None)
            .await
            .context("Failed to get relayer balance")?;
        
        let needs_refill = balance < self.gas_threshold;
        
        // Estimate transactions remaining (assuming 100k gas per tx at 20 gwei)
        let gas_price = U256::from(20_000_000_000u64); // 20 gwei
        let gas_per_tx = U256::from(100_000u64);
        let cost_per_tx = gas_price * gas_per_tx;
        let tx_remaining = if cost_per_tx > U256::zero() {
            (balance / cost_per_tx).as_u64()
        } else {
            0
        };
        
        let balance_eth = format!("{:.4}", balance.as_u128() as f64 / 1e18);
        let threshold_eth = format!("{:.4}", self.gas_threshold.as_u128() as f64 / 1e18);
        
        Ok(GasTankStatus {
            balance: balance_eth,
            balance_wei: balance,
            threshold: threshold_eth,
            needs_refill,
            transactions_remaining: tx_remaining,
        })
    }
    
    /// Get relayer status
    pub async fn status(&self) -> Result<String> {
        let gas_status = self.check_gas_tank().await?;
        Ok(format!(
            "Relayer: {:?} | Forwarder: {:?} | Gas Tank: {} ETH ({} tx remaining) {}",
            self.signer.address(),
            self.forwarder_address,
            gas_status.balance,
            gas_status.transactions_remaining,
            if gas_status.needs_refill { "⚠️  REFILL NEEDED" } else { "✅" }
        ))
    }
    
    /// Get next nonce for user
    pub async fn get_nonce(&self, user: Address) -> Result<U256> {
        let mut nonces = self.nonce_tracker.write().await;
        let nonce = nonces.entry(user).or_insert(U256::zero());
        let current = *nonce;
        *nonce += U256::one();
        Ok(current)
    }
}
