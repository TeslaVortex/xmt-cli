use ethers::prelude::*;
use anyhow::{Result, Context};
use std::sync::Arc;
use crate::contracts::xmoney::XMoney;
use crate::web3::Web3Provider;
use crate::web3::signer::WalletSigner;

pub struct XMoneyBridge {
    xmoney: XMoney<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
    signer_address: Address,
}

impl XMoneyBridge {
    pub async fn new(
        contract_address: Address,
        provider: &Web3Provider,
        signer: WalletSigner,
    ) -> Result<Self> {
        let signer_address = signer.address();
        let client = Arc::new(signer.with_provider(provider.provider()));
        let xmoney = XMoney::new(contract_address, client);

        Ok(Self {
            xmoney,
            signer_address,
        })
    }

    pub async fn mint(&self, to: Address, amount: U256) -> Result<TransactionReceipt> {
        tracing::info!(
            "Minting {} tokens to {} from bridge",
            amount,
            to
        );

        let receipt = self.xmoney.mint(to, amount).await
            .context("Bridge mint failed")?;

        tracing::info!(
            "Mint successful. Transaction hash: {:?}",
            receipt.transaction_hash
        );

        Ok(receipt)
    }

    pub async fn burn(&self, amount: U256) -> Result<TransactionReceipt> {
        tracing::info!(
            "Burning {} tokens from {}",
            amount,
            self.signer_address
        );

        let receipt = self.xmoney.burn(self.signer_address, amount).await
            .context("Bridge burn failed")?;

        tracing::info!(
            "Burn successful. Transaction hash: {:?}",
            receipt.transaction_hash
        );

        Ok(receipt)
    }

    pub async fn get_balance(&self, address: Address) -> Result<U256> {
        self.xmoney.balance_of(address).await
    }

    pub async fn get_total_supply(&self) -> Result<U256> {
        self.xmoney.total_supply().await
    }

    pub fn signer_address(&self) -> Address {
        self.signer_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_structure() {
        // Placeholder test - actual tests require deployed contracts
        assert!(true);
    }
}
