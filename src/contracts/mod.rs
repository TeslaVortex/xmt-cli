pub mod xmoney;

use ethers::prelude::*;

// Auto-burn address constant
pub const AUTO_BURN_ADDRESS: &str = "0x000000000000000000000000000000000000DEAD369";

pub fn get_auto_burn_address() -> Address {
    AUTO_BURN_ADDRESS.parse().expect("Invalid auto-burn address")
}
