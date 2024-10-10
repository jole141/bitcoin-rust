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

/// Calculates the merkle root of a list of transactions
/// by hashing pairs of transaction hashes until only one hash remains
pub fn calculate_merkle_root(transactions: &Vec<Transaction>) -> String {
    let mut hashes: Vec<String> = transactions.iter().map(|transaction| transaction.hash().to_string()).collect();
    while hashes.len() > 1 {
        let mut new_hashes: Vec<String> = vec![];
        for i in (0..hashes.len()).step_by(2) {
            let left = &hashes[i];
            let right = if i + 1 < hashes.len() {
                &hashes[i + 1]
            } else {
                &hashes[i]
            };
            let new_hash = sha256_hash(format!("{}{}", left, right).as_str());
            new_hashes.push(new_hash.to_string());
        }
        hashes.clear();
        hashes.extend(new_hashes);
    }
    hashes[0].clone()
}

/// Validates a block by checking if the hash of the block is correct
/// and if the merkle root of the block is correct
/// and if the timestamp of the block is in the past
/// and if the difficulty target of the block is correct
/// and if each transaction in the block is valid
pub fn validate_block(block: &Block) -> bool {
    let block_hash = sha256_hash(block.header.to_string().as_str());
    let transactions = &block.transactions;
    let merkle_root = calculate_merkle_root(transactions);
    if block.hash_block() != block_hash {
        return false;
    }
    // Check if the merkle root of the block is correct
    if merkle_root != block.header.merkle_root {
        return false;
    }
    // Check if the timestamp of the block is in the past
    if block.header.timestamp > get_current_timestamp_ms() {
        return false;
    }
    
    // TODO: difficulty target check
    // TODO: validate each transaction in the block
    // TODO: add other checks
    true
}


/// Initializes the genesis block
pub fn init_genesis_block(miner_pub_key: PublicKey) -> Block {
    let script_pub_key = miner_pub_key.to_string();
    let coinbase_transaction = Transaction::new_coinbase_transaction(script_pub_key, miner_pub_key.to_string());
    let transactions = vec![coinbase_transaction.clone()];
    let merkle_root = calculate_merkle_root(&transactions);
    let genesis_block = Block::new(
        SOFTWARE_VERSION.to_string(), 
        None, 
        merkle_root, 
        get_current_timestamp_ms(), 
        0, 
        0, 
        transactions, 
        coinbase_transaction
    );
    genesis_block
}

/// Validates a blockchain by checking if each block in the blockchain is valid
/// starts from the last block in the blockchain
pub fn validate_blockchain(blockchain: &Vec<Block>) -> bool {
    for i in (1..blockchain.len()).rev() {
        if !validate_block(&blockchain[i]) {
            return false;
        }
    }
    true
}

/// Mines a new block by creating a new block with a coinbase transaction
pub fn mine_new_block(miner_pub_key: PublicKey,previous_block_hash: &sha256::Hash, transactions: Vec<Transaction>) -> Block {
    let script_pub_key = miner_pub_key.to_string();
    let coinbase_transaction = Transaction::new_coinbase_transaction(script_pub_key, miner_pub_key.to_string());
    let mut all_transactions = vec![coinbase_transaction.clone()];
    all_transactions.extend(transactions);
    let merkle_root = calculate_merkle_root(&all_transactions);
    let new_block = Block::new(
        SOFTWARE_VERSION.to_string(), 
        Some(previous_block_hash.to_string()), 
        merkle_root, 
        get_current_timestamp_ms(), 
        0, 
        0, 
        all_transactions, 
        coinbase_transaction
    );

    new_block
}