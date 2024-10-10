use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use secp256k1::{PublicKey, SecretKey};

use crate::constants::NUMBER_OF_NODES;
use crate::core::block::{init_genesis_block, mine_new_block, Block};
use crate::utils;

use super::block::validate_blockchain;
use super::transaction::Transaction;

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
                        let genesis_block = init_genesis_block(self.pub_key);
                        blockchain.push(genesis_block.clone());
                        println!("#{} ({}) (Genesis block) -> mined by #{} node (pubKey: {})", blockchain.len(),genesis_block.hash_block(), self.id, self.pub_key);
                    } else {
                        let previous_block_hash = blockchain.last().unwrap().hash_block();
                        let new_transactions = get_list_of_transactions();
                        let new_block = mine_new_block(self.pub_key, &previous_block_hash, new_transactions);
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
                    if validate_blockchain(&new_blockchain) {
                        blockchain.push(new_block.clone());
                        println!("New block got accepted by #{} node", self.id);
                    } else {
                        println!("Received block is invalid!");
                    }
                }
            }
        });
    }
}

/// Get available transactions to be included in a block
/// Temporary function to return an empty list of transactions
fn get_list_of_transactions() -> Vec<Transaction> {
    vec![]
}