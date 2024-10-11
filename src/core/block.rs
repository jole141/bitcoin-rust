use core::fmt;

use secp256k1::hashes::sha256;
use secp256k1::PublicKey;

use crate::core::transaction::Transaction;
use crate::constants::SOFTWARE_VERSION;
use crate::utils::hash::sha256_hash;
use crate::utils::time::get_current_timestamp_ms;

#[derive(Debug, Clone)]
pub struct Block {
    /// The block header contains metadata about the block
    pub header: BlockHeader,
    /// The list of transactions in the block
    pub transactions: Vec<Transaction>,
    /// The first transaction in the block
    /// has no inputs, and is called the coinbase transaction
    /// reward for the miner
    pub coinbase_transaction: Transaction,
}

impl Block {
    pub fn new(software_version: String, previous_block_hash: Option<String>, merkle_root: String, timestamp: u128, difficulty_target: u32, nonce: u32, transactions: Vec<Transaction>, coinbase_transaction: Transaction) -> Block {
        Block {
            header: BlockHeader {
                software_version,
                previous_block_hash,
                merkle_root,
                timestamp,
                difficulty_target,
                nonce,
            },
            transactions,
            coinbase_transaction,
        }
    }

    pub fn hash_block(&self) -> sha256::Hash {
        sha256_hash(self.header.to_string().as_str())
    }
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    /// The version of the block
    pub software_version: String,
    /// The hash of the previous block
    pub previous_block_hash: Option<String>,
    /// The root of the merkle tree of transactions
    pub merkle_root: String,
    /// The time of the block creation
    pub timestamp: u128,
    /// The target value for the block hash
    /// Number of leading zeros in the hash
    pub difficulty_target: u32,
    /// The nonce value that miners increment
    pub nonce: u32,
}

impl fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BlockHeader({:?})", self)
    }
}