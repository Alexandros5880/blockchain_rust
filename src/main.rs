mod transaction;
mod block;
mod blockchain;
use blockchain::Blockchain;

fn main() {

    /*
        Mining creates new blocks
     */

    // Instantiate the Blockchain
    // Block 1
    let mut blockchain = Blockchain::new();

    // Create a new transactions
    blockchain.new_transaction("Alice", "Antonys", 1000000000);
    blockchain.new_transaction("Alice", "Alexandros", 1000000000);
    blockchain.new_transaction("Alice", "Haris", 1000000000);
    blockchain.new_transaction("Alice", "Mirela", 1000000000);

    // Mine the block
    let last_rc_block = blockchain.last_block().clone();
    let last_block = last_rc_block.borrow();
    let last_proof = last_block.proof;
    let proof = blockchain.proof_of_work(last_proof);
    println!("\nproof: {:#?}", proof);



    // Forge the new block by adding it to the chain
    // Block 2
    let previous_hash = last_block.hash();
    blockchain.new_block(proof, Some(previous_hash));

    // Create a new transactions
    blockchain.new_transaction("Giorgos", "Antonys", 1000000000);
    blockchain.new_transaction("Giorgos", "Alexandros", 1000000000);
    blockchain.new_transaction("Giorgos", "Haris", 1000000000);
    blockchain.new_transaction("Giorgos", "Mirela", 1000000000);

    // Mine the block
    let last_rc_block = blockchain.last_block();
    let last_block = last_rc_block.borrow();
    let last_proof = last_block.proof;
    let proof = blockchain.proof_of_work(last_proof);
    println!("\nproof: {:#?}", proof);


    // Print the full blockchain
    println!("\n\n{:#?}", blockchain);

}
