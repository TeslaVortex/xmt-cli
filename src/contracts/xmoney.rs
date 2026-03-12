use ethers::prelude::*;
use anyhow::{Result, Context};
use std::sync::Arc;

// X-Money contract ABI - v2 with AccessControl
abigen!(
    XMoneyContract,
    r#"[
        function mint(address to, uint256 amount) external
        function burn(address from, uint256 amount) external
        function balanceOf(address account) external view returns (uint256)
        function totalSupply() external view returns (uint256)
        function hasRole(bytes32 role, address account) external view returns (bool)
        function grantRole(bytes32 role, address account) external
        function revokeRole(bytes32 role, address account) external
        function getRoleAdmin(bytes32 role) external view returns (bytes32)
        function DEFAULT_ADMIN_ROLE() external view returns (bytes32)
        function MINTER_ROLE() external view returns (bytes32)
        function BURNER_ROLE() external view returns (bytes32)
        function APEX_936() external view returns (uint256)
        function VORTEX_369() external view returns (uint256)
        function CODE_66() external view returns (uint256)
        function getSacredNumbers() external view returns (uint256, uint256, uint256)
        event Transfer(address indexed from, address indexed to, uint256 value)
        event TokensMinted(address indexed to, uint256 amount, uint256 timestamp)
        event TokensBurned(address indexed from, uint256 amount, uint256 timestamp)
        event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender)
        event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender)
    ]"#,
);

pub struct XMoney<M: Middleware> {
    contract: XMoneyContract<M>,
}

impl<M: Middleware + 'static> XMoney<M> {
    pub fn new(contract_address: Address, client: Arc<M>) -> Self {
        let contract = XMoneyContract::new(contract_address, client);
        Self { contract }
    }

    pub async fn mint(&self, to: Address, amount: U256) -> Result<TransactionReceipt> {
        let tx = self.contract.mint(to, amount);
        
        let pending_tx = tx
            .send()
            .await
            .context("Failed to send mint transaction")?;

        let receipt = pending_tx
            .await
            .context("Failed to get transaction receipt")?
            .context("Transaction failed")?;

        Ok(receipt)
    }

    pub async fn burn(&self, from: Address, amount: U256) -> Result<TransactionReceipt> {
        let tx = self.contract.burn(from, amount);
        
        let pending_tx = tx
            .send()
            .await
            .context("Failed to send burn transaction")?;

        let receipt = pending_tx
            .await
            .context("Failed to get transaction receipt")?
            .context("Transaction failed")?;

        Ok(receipt)
    }

    pub async fn balance_of(&self, account: Address) -> Result<U256> {
        self.contract
            .balance_of(account)
            .call()
            .await
            .context("Failed to get balance")
    }

    pub async fn total_supply(&self) -> Result<U256> {
        self.contract
            .total_supply()
            .call()
            .await
            .context("Failed to get total supply")
    }
}
