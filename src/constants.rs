/// Bitcoin constants
pub const SOFTWARE_VERSION: &str = "0.1.0";
pub const TX_VERSION: u32 = 1;
pub const COINBASE_VALUE: u128 = 50_000_000_000; // 50 BTC

pub const AVERAGE_BLOCK_TIME_MS: u64 = 5000; 

// TODO: use secp256k1 crate to generate a public key
pub const MY_PUB_ADDRESS: &str = "MY_PUB_ADDRESS";
