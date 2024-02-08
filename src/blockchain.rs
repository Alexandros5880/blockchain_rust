use sha2::{ Sha256, Digest }; // Sha512
use std::cell::RefCell;
use std::rc::Rc;
use crate::transaction::Transaction;
use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Rc<RefCell<Block>>>,
    current_transactions: Vec<Transaction>,
}

impl Blockchain {

    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
        };
        blockchain.new_block(100, None);
        blockchain
    }

    pub fn new_block(&mut self, proof: usize, previous_hash: Option<String>) -> Rc<RefCell<Block>> {
        let block = Block::new(previous_hash, proof);
        self.chain.push(Rc::from(RefCell::new(block)));
        self.last_block()
    }

    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: i32) -> usize {
        let transaction = Transaction {
            sender: String::from(sender),
            recipient: String::from(recipient),
            amount,
        };
        self.current_transactions.push(transaction.clone());
        let last_rc_block = self.last_block();
        let mut last_block = last_rc_block.borrow_mut();
        let _ = last_block.index + 1;
        last_block.transactions.push(transaction);
        last_block.index.clone()
    }

    pub fn last_block(&self) -> Rc<RefCell<Block>> {
        self.chain[self.chain.len()-1].to_owned()
    }

    pub fn proof_of_work(&self, last_proof: usize) -> usize {
        let mut proof = 0;
        while !self.valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    fn valid_proof(&self, last_proof: usize, proof: usize) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = self.hash_string(&guess);
        guess_hash.starts_with("0000")
    }

    fn hash_string(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:X}", hasher.finalize())
    }

}
