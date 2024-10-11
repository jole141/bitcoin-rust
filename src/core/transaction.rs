use std::fmt;

use secp256k1::hashes::sha256;
use secp256k1::PublicKey;

use crate::constants::{COINBASE_VALUE, TX_VERSION};
use crate::utils::hash::sha256_hash;

#[derive(Debug, Clone)]
pub struct Transaction {
    /// The version of the transaction
    pub transaction_version: u32,
    /// The number of inputs in the transaction
    pub input_count: u32,
    /// The list of inputs in the transaction
    pub inputs: Vec<TransactionInput>,
    /// The number of outputs in the transaction
    pub output_count: u32,
    /// The list of outputs in the transaction
    pub outputs: Vec<TransactionOutput>,
    /// The time of the transaction creation
    pub lock_time: u32,
}

impl Transaction {
    pub fn new_coinbase_transaction(script_pub_key: String, recipient_pub_key: PublicKey) -> Transaction {
        Transaction {
            transaction_version: TX_VERSION,
            input_count: 0,
            inputs: vec![],
            output_count: 1,
            outputs: vec![
                TransactionOutput {
                    value: COINBASE_VALUE,
                    script_length: 0,
                    script_pub_key,
                    recipient_pub_key,
                }
            ],
            lock_time: 0,
        }
    }

    pub fn hash(&self) -> sha256::Hash {
        sha256_hash(self.to_string().as_str())
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction({:?})", self)
    }
}

#[derive(Debug, Clone)]
pub struct TransactionInput {
    /// The hash of the previous transaction
    pub previous_transaction_hash: String,
    /// The index of the previous transaction
    pub previous_transaction_index: u32,
    /// The length of the scriptSig field
    pub script_length: u32,
    /// The signature script
    pub script_sig: String,
    /// Number that miners use for transaction blocking
    /// (to prevent the same transaction from being included in the block multiple times)
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct TransactionOutput {
    /// The number of satoshis to be transfered (1 BTC = 10^9 satoshis)
    pub value: u128,
    /// The length of the scriptPubKey field
    pub script_length: u32,
    /// The public key script
    pub script_pub_key: String,
    /// The address of the recipient (public key hash)
    /// used to make the transaction more human-readable
    pub recipient_pub_key: PublicKey,
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