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
pub fn calculate_merkle_root(transactions: &Vec<Transaction>) -> sha256::Hash {
    let mut hashes: Vec<sha256::Hash> = transactions.iter().map(|transaction| transaction.hash()).collect();
    while hashes.len() > 1 {
        let mut new_hashes: Vec<sha256::Hash> = vec![];
        for i in (0..hashes.len()).step_by(2) {
            let left = &hashes[i];
            let right = if i + 1 < hashes.len() {
                &hashes[i + 1]
            } else {
                &hashes[i]
            };
            let new_hash = sha256_hash(format!("{}{}", left, right).as_str());
            new_hashes.push(new_hash);
        }
        hashes.clear();
        hashes.extend(new_hashes);
    }
    hashes[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::hashes::Hash;
    use secp256k1::Secp256k1;
    use secp256k1::rand::rngs::OsRng;

    fn generate_public_key() -> PublicKey {
        let secp = Secp256k1::new();
        let (_, public_key) = secp.generate_keypair(&mut OsRng);
        public_key
    }

    #[test]
    fn test_new_coinbase_transaction() {
        let pub_key = generate_public_key();
        let script_pub_key = "76a914...88ac".to_string(); // Pseudo scriptPubKey

        let tx = Transaction::new_coinbase_transaction(script_pub_key.clone(), pub_key.clone());

        assert_eq!(tx.transaction_version, TX_VERSION);
        assert_eq!(tx.input_count, 0);
        assert_eq!(tx.inputs.len(), 0);
        assert_eq!(tx.output_count, 1);
        assert_eq!(tx.outputs.len(), 1);
        assert_eq!(tx.outputs[0].value, COINBASE_VALUE);
        assert_eq!(tx.outputs[0].script_pub_key, script_pub_key);
        assert_eq!(tx.outputs[0].recipient_pub_key, pub_key);
    }

    #[test]
    fn test_transaction_hash() {
        let pub_key = generate_public_key();
        let script_pub_key = "76a914...88ac".to_string();

        let tx = Transaction::new_coinbase_transaction(script_pub_key, pub_key);

        // check if the hash is 32 bytes long
        let hash = tx.hash();
        assert_eq!(hash.to_byte_array().len(), 32);  // 32 bytes
    }

    #[test]
    fn test_calculate_merkle_root() {
        let pub_key = generate_public_key();
        let script_pub_key = "76a914...88ac".to_string();

        // create 3 coinbase transactions
        let tx1 = Transaction::new_coinbase_transaction(script_pub_key.clone(), pub_key.clone());
        let tx2 = Transaction::new_coinbase_transaction(script_pub_key.clone(), pub_key.clone());
        let tx3 = Transaction::new_coinbase_transaction(script_pub_key.clone(), pub_key.clone());

        let transactions = vec![tx1, tx2, tx3];
        let merkle_root = calculate_merkle_root(&transactions);

        // check if the merkle root is 32 bytes long
        assert_eq!(merkle_root.as_byte_array().len(), 32);  // 64 hex characters = 32 bytes
    }
}
