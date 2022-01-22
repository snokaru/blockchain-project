use hex;
use blockchain::{Blockchain, Transaction};
use serde_json;

fn main() {
    let mut blockchain = Blockchain::new();

    let transaction = Transaction::new("Vlad Slivilescu".to_string(), "Ionut Popescu".to_string(), 550);
    blockchain.add_transaction(transaction);
    blockchain.generate_block();

    let first_block = blockchain.get_block(0).unwrap();
    let second_block = blockchain.get_block(1).unwrap();
    let json = serde_json::to_string_pretty(second_block).unwrap();
    println!("{}", json);
    println!("{}", hex::encode(first_block.hash()));
}
