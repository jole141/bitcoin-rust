mod core;
mod constants;
mod utils;

use rand::Rng;

use core::{block::Block, consensus::Node};
use std::{sync::{mpsc, Arc, Mutex}, time::Duration};
use constants::{AVERAGE_BLOCK_TIME_MS, NUMBER_OF_NODES};


fn main() {
    multithreaded_blockchain();
    
}

fn multithreaded_blockchain() {
    let mut tx_channels = vec![];
    let mut node_threads = vec![];
    let tx_rx_channels = Arc::new(Mutex::new(vec![]));

    // Creates channels for syncing blocks between nodes
    for _  in 0..NUMBER_OF_NODES {
        let (tx_block, rx_block) = mpsc::channel::<Block>();
        tx_rx_channels.lock().unwrap().push((tx_block, rx_block));
        
    }

    // Creating NUMBER_OF_NODES threads to simulate nodes
    for id in 0..NUMBER_OF_NODES {
        // channel for picking random node to mine a block
        let (tx, rx) = mpsc::channel::<u32>();
        // clone tx_rx_channels to be used in the thread
        let tx_rx_channels_clone = Arc::clone(&tx_rx_channels);
        let node = Arc::new(Node::new(id));
        let thread = node.start_node(rx, tx_rx_channels_clone);
        node_threads.push(thread);
        tx_channels.push(tx);
    }

    // Main loop to simulate mining blocks (pick a random node to mine a block)
    loop { 
        let random_node_id: u32 = rand::thread_rng().gen_range(0..NUMBER_OF_NODES);
        println!("KBV picked a random node id: {}", random_node_id);
        let choosen_tx = &tx_channels[random_node_id as usize];
        choosen_tx.send(random_node_id).unwrap();
        std::thread::sleep(Duration::from_millis(AVERAGE_BLOCK_TIME_MS));
        println!("------------------------------------");
    }
}
