use hex;
use std::convert::TryInto;
use sha2::{Sha256, Digest};
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize, Serializer};
use serde::ser::{SerializeStruct};
use bincode;
use rand::Rng;
use k256::{
    ecdsa::{SigningKey, Signature, signature::{Signer, Verifier}, VerifyingKey}
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnsignedTransaction {
    sender: VerifyingKey,
    receiver: VerifyingKey,
    amount: i128,
    timestamp: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    sender: VerifyingKey,
    receiver: VerifyingKey,
    amount: i128,
    timestamp: DateTime<Local>,
    signature: Signature,
}

impl Transaction {
    pub fn new(private_key: SigningKey, public_key: VerifyingKey, receiver: VerifyingKey, amount: i128) -> Transaction {
        let timestamp = Local::now();

        let transaction = UnsignedTransaction {
            sender: public_key.clone(),
            receiver: receiver.clone(),
            amount,
            timestamp,
        };

        let serialized = bincode::serialize(&transaction).unwrap();
        let signature = private_key.sign(&serialized[..]);

        Transaction {
            sender: public_key,
            receiver,
            amount,
            timestamp,
            signature,
        }
    }

    pub fn verify(&self) -> bool {
        let transaction = UnsignedTransaction {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            amount: self.amount.clone(),
            timestamp: self.timestamp.clone(),
        };

        let verifying_key = VerifyingKey::from(transaction.sender.clone());
        let serialized = bincode::serialize(&transaction).unwrap();
        match verifying_key.verify(&serialized[..], &self.signature) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Block {
    height: usize,
    timestamp: DateTime<Local>,
    transactions: Vec<Transaction>,
    pub previous_hash: [u8; 32],
    target: [u8; 32],
    nonce: u32,
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Color", 5)?;
        state.serialize_field("index", &self.height)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("transactions", &self.transactions)?;
        state.serialize_field("previous_hash", &hex::encode(self.previous_hash))?;
        state.end()
    }
}

impl Block {
    pub fn new(height: usize, previous_hash: [u8; 32], target: [u8; 32]) -> Block {
        let mut rng = rand::thread_rng();
        Block {
            height,
            timestamp: Local::now(),
            transactions: Vec::<Transaction>::new(),
            target,
            previous_hash,
            nonce: rng.gen(),
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(bincode::serialize(self).unwrap());
        return hasher.finalize().into();
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn is_valid(&self) -> bool {
        hex::encode(self.hash()).cmp(&hex::encode(&self.target)) == std::cmp::Ordering::Less
    }

    pub fn verify(&self) -> bool {
        for transaction in self.transactions.iter() {
            if !transaction.verify() {
                return false;
            }
        }
        return true;
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    target: [u8; 32],
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::<Block>::new(),
            pending_transactions: Vec::<Transaction>::new(),
            target: hex::decode("0000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap().try_into().unwrap(),
        };
        blockchain.generate_block();
        blockchain
    }
    
    fn generate_block(&mut self) {
        let height = self.chain.len();
        let previous_block = if height > 0 { self.chain.get(height - 1) } else { None };

        let previous_hash = match previous_block {
            None => [0; 32],
            Some(block) => block.hash(),
        };

        loop {
            let mut new_block = Block::new(height, previous_hash, self.target.clone()); 
            for transaction in self.pending_transactions.iter() {
                new_block.add_transaction(transaction.clone());
            }
            if new_block.is_valid() { 
                self.chain.push(new_block);
                self.pending_transactions = Vec::new();
                break; 
            }
        }
    }

    pub async fn mine_block(&mut self) {
        self.generate_block();
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        return self.chain.get(self.chain.len() - 1);
    }

    pub fn get_block(&self, idx: usize) -> Option<&Block> {
        return self.chain.get(idx);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }
}
