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
    pub fn new(software_version: String, previous_block_hash: Option<sha256::Hash>, merkle_root: sha256::Hash, timestamp: u128, difficulty_target: u32, nonce: u32, transactions: Vec<Transaction>, coinbase_transaction: Transaction) -> Block {
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
    pub previous_block_hash: Option<sha256::Hash>,
    /// The root of the merkle tree of transactions
    pub merkle_root: sha256::Hash,
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

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::hashes::Hash;
    use secp256k1::Secp256k1;
    use secp256k1::rand::rngs::OsRng;
    use crate::core::transaction::Transaction;
    use crate::utils::time::get_current_timestamp_ms;

    const DUMMY_NONCE: u32 = 1234567;
    const DUMMY_DIFFICULTY_TARGET: u32 = 0xabcdef12;

    fn generate_public_key() -> PublicKey {
        let secp = Secp256k1::new();
        let (_, public_key) = secp.generate_keypair(&mut OsRng);
        public_key
    }

    fn get_dummy_merkle_root() -> sha256::Hash {
        sha256_hash("dummy_merkle_root")
    }

    fn generate_dummy_previous_block_hash() -> sha256::Hash {
        sha256_hash("dummy_previous_block_hash")
    }

    fn create_dummy_transaction() -> Transaction {
        let pub_key = generate_public_key();
        let script_pub_key = "76a914...88ac".to_string();

        Transaction::new_coinbase_transaction(script_pub_key, pub_key)
    }

    #[test]
    fn test_new_block_creation() {
        let dummy_previous_block_hash = generate_dummy_previous_block_hash();
        let dummy_merkle_root = get_dummy_merkle_root();
        let dummy_transaction = create_dummy_transaction();
        let transactions = vec![dummy_transaction.clone()];
        let coinbase_transaction = dummy_transaction.clone();
        let software_version = SOFTWARE_VERSION.to_string();
        let timestamp = get_current_timestamp_ms();

        let block = Block::new(
            software_version.clone(),
            Some(dummy_previous_block_hash),
            dummy_merkle_root,
            timestamp,
            DUMMY_DIFFICULTY_TARGET,
            DUMMY_NONCE,
            transactions.clone(),
            coinbase_transaction.clone(),
        );

        assert_eq!(block.header.software_version, software_version);
        assert_eq!(block.header.previous_block_hash, Some(dummy_previous_block_hash));
        assert_eq!(block.header.merkle_root, dummy_merkle_root);
        assert_eq!(block.header.timestamp, timestamp);
        assert_eq!(block.header.difficulty_target, DUMMY_DIFFICULTY_TARGET);
        assert_eq!(block.header.nonce, DUMMY_NONCE);
        assert_eq!(block.transactions.len(), 1);
    }

    #[test]
    fn test_hash_block() {
        let dummy_previous_block_hash = generate_dummy_previous_block_hash();
        let dummy_merkle_root = get_dummy_merkle_root();
        let dummy_transaction = create_dummy_transaction();
        let transactions = vec![dummy_transaction.clone()];
        let coinbase_transaction = dummy_transaction.clone();
        let block = Block::new(
            SOFTWARE_VERSION.to_string(),
            Some(dummy_previous_block_hash),
            dummy_merkle_root,
            get_current_timestamp_ms(),
            DUMMY_DIFFICULTY_TARGET,
            DUMMY_NONCE,
            transactions,
            coinbase_transaction,
        );

        let block_hash = block.hash_block();
        assert_eq!(block_hash.as_byte_array().len(), 32);  // SHA-256 hash bi trebao imati 32 bajta
    }

    #[test]
    fn test_block_header_display() {
        let dummy_previous_block_hash = generate_dummy_previous_block_hash();
        let dummy_merkle_root = get_dummy_merkle_root();
        let block_header = BlockHeader {
            software_version: SOFTWARE_VERSION.to_string(),
            previous_block_hash: Some(dummy_previous_block_hash),
            merkle_root: dummy_merkle_root,
            timestamp: get_current_timestamp_ms(),
            difficulty_target: DUMMY_DIFFICULTY_TARGET,
            nonce: DUMMY_NONCE,
        };

        let header_display = format!("{}", block_header);
        assert!(header_display.contains("BlockHeader"));
        assert!(header_display.contains("software_version"));
        assert!(header_display.contains("previous_block_hash"));
        assert!(header_display.contains("merkle_root"));
        assert!(header_display.contains("timestamp"));
        assert!(header_display.contains("difficulty_target"));
        assert!(header_display.contains("nonce"));
    }
}
