use hex;
use sha2::{Sha256, Digest};
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize, Serializer};
use serde::ser::{SerializeStruct};
use bincode;


#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: i128,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: i128) -> Transaction {
        Transaction {
            sender,
            recipient,
            amount
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Block {
    index: usize,
    timestamp: DateTime<Local>,
    transactions: Vec<Transaction>,
    previous_hash: [u8; 32],
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("transactions", &self.transactions)?;
        state.serialize_field("previous_hash", &hex::encode(self.previous_hash))?;
        state.end()
    }
}

impl Block {
    pub fn new(index: usize, previous_hash: [u8; 32]) -> Block {
        Block {
            index,
            timestamp: Local::now(),
            transactions: Vec::<Transaction>::new(),
            previous_hash,
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

}


#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::<Block>::new(),
            pending_transactions: Vec::<Transaction>::new(),
        };
        blockchain.generate_block();
        blockchain
    }
    
    pub fn generate_block(&mut self) {
        let index = self.chain.len();
        let previous_block = if index > 0 { self.chain.get(index - 1) } else { None };

        let previous_hash = match previous_block {
            None => [0; 32],
            Some(block) => block.hash(),
        };

        let mut new_block = Block::new(index, previous_hash); 
        while let Some(transaction) = self.pending_transactions.pop() {
            new_block.add_transaction(transaction);
        }
        self.chain.push(new_block);
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
