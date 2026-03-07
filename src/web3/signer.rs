use ethers::prelude::*;
use anyhow::{Result, Context};
use std::sync::Arc;

pub struct WalletSigner {
    wallet: LocalWallet,
    chain_id: u64,
}

impl WalletSigner {
    pub fn new(private_key: &str, chain_id: u64) -> Result<Self> {
        let wallet = private_key
            .parse::<LocalWallet>()
            .context("Failed to parse private key")?
            .with_chain_id(chain_id);

        Ok(Self { wallet, chain_id })
    }

    pub fn address(&self) -> Address {
        self.wallet.address()
    }

    pub fn wallet(&self) -> &LocalWallet {
        &self.wallet
    }

    pub fn with_provider<P: JsonRpcClient>(
        self,
        provider: Arc<Provider<P>>,
    ) -> SignerMiddleware<Arc<Provider<P>>, LocalWallet> {
        SignerMiddleware::new(provider, self.wallet)
    }

    pub async fn sign_message(&self, message: &[u8]) -> Result<Signature> {
        self.wallet
            .sign_message(message)
            .await
            .context("Failed to sign message")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        // Test with a dummy private key (32 bytes of zeros)
        let dummy_key = "0x0000000000000000000000000000000000000000000000000000000000000001";
        let signer = WalletSigner::new(dummy_key, 8453);
        assert!(signer.is_ok());
    }
}
