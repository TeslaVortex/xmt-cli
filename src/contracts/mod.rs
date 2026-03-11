pub mod xmoney;
pub mod vector_registry;
pub mod vector_minter;

use ethers::prelude::*;

// Auto-burn address constant
pub const AUTO_BURN_ADDRESS: &str = "0x000000000000000000000000000000000000dEaD";

pub fn get_auto_burn_address() -> Address {
    AUTO_BURN_ADDRESS.parse().expect("Invalid auto-burn address")
}
