use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use secp256k1::hashes::sha256;
use secp256k1::{PublicKey, SecretKey};

use crate::constants::{NUMBER_OF_NODES, SOFTWARE_VERSION};
use crate::core::block::Block;
use crate::utils;
use crate::utils::hash::sha256_hash;
use crate::utils::time::get_current_timestamp_ms;
use super::transaction::{calculate_merkle_root, Transaction};

/// Node struct represents a node in the network
pub struct Node {
    pub id: u32,
    pub pub_key: PublicKey,
    secret_key: SecretKey,
    blockchain: Mutex<Vec<Block>>,
}

impl Node {
    pub fn new(id: u32) -> Node {
        let (secret_key, public_key) = utils::wallets::generate_keypair();
        Node {
            id,
            pub_key: public_key,
            secret_key: secret_key,
            blockchain: Mutex::new(vec![]),
        }
    }

    /// Start the node (thread) and listen for incoming messages
    /// handles being a miner and receiving blocks from other nodes
    pub fn start_node(self: Arc<Self>, rx: Receiver<u32>, tx_rx_channels_clone: Arc<Mutex<Vec<(Sender<Block>, Receiver<Block>)>>>) {
        std::thread::spawn(move || {
            loop {
                // waiting for a message from the main thread (random node id)
                if let Ok(_) = rx.try_recv() {
                    let mut blockchain = self.blockchain.lock().unwrap();

                    if blockchain.is_empty() {
                        let genesis_block = Self::init_genesis_block(self.pub_key);
                        blockchain.push(genesis_block.clone());
                        println!("#{} ({}) (Genesis block) -> mined by #{} node (pubKey: {})", blockchain.len(),genesis_block.hash_block(), self.id, self.pub_key);
                    } else {
                        let previous_block_hash = blockchain.last().unwrap().hash_block();
                        let new_transactions = get_list_of_transactions();
                        let new_block = Self::mine_new_block(self.pub_key, &previous_block_hash, new_transactions);
                        blockchain.push(new_block.clone());
                        println!("#{} block ({}) -> mined by #{} node (pubKey: {})", blockchain.len(), new_block.hash_block(), self.id, self.pub_key);
                    }
                    // sending block to all other nodes
                    let tx_rx_channels = tx_rx_channels_clone.lock().unwrap();
                    for i in 0..NUMBER_OF_NODES {
                        if i != self.id {
                            let (tx_block, _) = &tx_rx_channels[i as usize];
                            tx_block.send(blockchain.last().unwrap().clone()).unwrap();
                        }
                    }
                }
    
                // waiting for a "block" from another node
                let tx_rx_channels = tx_rx_channels_clone.lock().unwrap();
                let (_, rx_block) = &tx_rx_channels[self.id as usize];
                if let Ok(new_block) = rx_block.try_recv() {
                    let mut blockchain = self.blockchain.lock().unwrap();
                    // copy the blockchain and add new block to the copied blockchain
                    let mut new_blockchain = blockchain.clone();
                    new_blockchain.push(new_block.clone());
                    if Node::validate_blockchain(&new_blockchain.clone()) {
                        blockchain.push(new_block.clone());
                        println!("New block got accepted by #{} node", self.id);
                    } else {
                        println!("Received block is invalid!");
                    }
                }
            }
        });
    }

    /// Initializes the genesis block
    pub fn init_genesis_block(miner_pub_key: PublicKey) -> Block {
        let script_pub_key = miner_pub_key.to_string();
        let coinbase_transaction = Transaction::new_coinbase_transaction(script_pub_key, miner_pub_key);
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

    /// Mines a new block by creating a new block with a coinbase transaction
    pub fn mine_new_block(miner_pub_key: PublicKey,previous_block_hash: &sha256::Hash, transactions: Vec<Transaction>) -> Block{
        let script_pub_key = miner_pub_key.to_string();
        let coinbase_transaction = Transaction::new_coinbase_transaction(script_pub_key, miner_pub_key);       
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

    /// Validates a blockchain by checking if each block in the blockchain is valid
    /// starts from the last block in the blockchain
    pub fn validate_blockchain(blockchain: &Vec<Block>) -> bool {
            for i in (1..blockchain.len()).rev() {
                if !Node::validate_block(&blockchain[i]) {
                    return false;
                }
            }
        true
    }
}

/// Get available transactions to be included in a block
/// Temporary function to return an empty list of transactions
fn get_list_of_transactions() -> Vec<Transaction> {
    vec![]
}