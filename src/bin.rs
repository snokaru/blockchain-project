use hex;
use blockchain::{Blockchain, Transaction};
use serde_json;
use rand_core::OsRng;
use k256::{
    ecdsa::{SigningKey}
};

#[tokio::main]
async fn main() {
    let mut blockchain = Blockchain::new();

    let sender_signing_key = SigningKey::random(&mut OsRng);
    let receiver_signing_key = SigningKey::random(&mut OsRng);

    let transaction = Transaction::new(sender_signing_key.clone(), sender_signing_key.verifying_key(), receiver_signing_key.verifying_key(), 1000);
    blockchain.add_transaction(transaction);
    blockchain.mine_block().await;

    let first_block = blockchain.get_block(0).unwrap();
    let first_block_json = serde_json::to_string_pretty(first_block).unwrap();
    println!("First block: ");
    println!("{}", first_block_json);
    println!();

    println!("Second block: ");
    let second_block = blockchain.get_block(1).unwrap();
    let second_block_json = serde_json::to_string_pretty(second_block).unwrap();
    println!("{}", second_block_json);
    println!();

    println!("Does the previous block hash of the second block match the hash of the first block?");
    println!("{} == {}", hex::encode(second_block.previous_hash), hex::encode(first_block.hash()));

    println!("Have all transactions in the second block been signed correctly using ECDSA?");
    if second_block.verify() {
        println!("Yes!");
    }
    else {
        println!("No!");
    }
}
