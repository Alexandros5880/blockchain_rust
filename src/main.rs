// use std::cell::RefCell;
mod transaction;
mod block;
mod blockchain;
use blockchain::Blockchain;
// use block::Block;

fn main() {
    // Instantiate the Blockchain
    let mut blockchain = Blockchain::new();

    // Create a new transaction
    blockchain.new_transaction("Alice", "Bob", 10);

    // Mine the block
    let last_block = blockchain.last_block();
    let last_proof = last_block.proof;
    let proof = blockchain.proof_of_work(last_proof);

    // Forge the new block by adding it to the chain
    let previous_hash = last_block.hash();
    blockchain.new_block(proof, Some(previous_hash));

    // Print the full blockchain
    println!("{:#?}", blockchain.chain);
}
