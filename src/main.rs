mod core;
mod constants;
mod utils;

use core::{block::{mine_new_block, validate_blockchain, Block}, transaction::Transaction};

use constants::AVERAGE_BLOCK_TIME_MS;

use crate::core::block::init_genesis_block;

fn main() {
    println!("Bitcoin in Rust!");
    let mut blockchain: Vec<Block> = vec![];

    let genesis_block = init_genesis_block();
    blockchain.push(genesis_block.clone());
    let mut block_number = 1;
    println!("#{} block: {}", block_number, genesis_block.hash_block());
    // simulate mining a new block
    let mut previous_block_hash = genesis_block.hash_block();
    loop {
        // simulate AVERAGE_BLOCK_TIME_MS seconds of mining
        std::thread::sleep(std::time::Duration::from_millis(AVERAGE_BLOCK_TIME_MS));
        let new_transactions = get_list_of_transactions();
        let new_block = mine_new_block(&previous_block_hash, new_transactions);
        // copy the blockchain and add new block to the copied blockchain
        let mut new_blockchain = blockchain.clone();
        new_blockchain.push(new_block.clone());
        if validate_blockchain(&new_blockchain) {
            println!("#{} block: {}", block_number+1, new_block.hash_block());
            previous_block_hash = new_block.hash_block();
            block_number += 1;
        } else {
            println!("Block is invalid!");
        }
    }
}

/// Get available transactions to be included in a block
fn get_list_of_transactions() -> Vec<Transaction> {
    vec![]
}