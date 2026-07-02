   // -------------------------------------------
   // 			Blockchain in Rust from Scratch 
   // ------------------------------------------- 

   #[derive(Debug, Clone)] 
   struct BlockChain {
    blocks: Vec<Block>,
   }

   #[derive(Debug, Clone)] 
   struct Block {
    id: u64, 
    nonce: u64, 
    data: String, 
    hash: String, 
    previous_hash: String, 
    timestamp: i64, 
   }

   impl BlockChain {
    fn new() -> Self {
        Self {
            blocks: vec![],
        }
    }

    fn starting_block(&mut self) {
        let genesis_block = Block {
            id: 1,
            data: String::from("I am a first or genesis block"), 
            previous_hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"), 
            nonce: 11316, 
            hash: String::from("000015783b764259d382017d91a36d206d0600e2cbb3567748f46a33fe9297cf"),
            timestamp:  Utc::now().timestamp(),
        }; 
        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        match self.blocks.last() {
            None => {
                println!("the blockchain does not have atleast one block"); 
                return; 
            }, 

            Some(latest_block) => {
                if self.is_block_valid(&block, latest_block) {
                    self.blocks.push(block); 
                    println!("Block has been successfully added"); 

                } else  {
                    println!("Could not add block, invalid!");
                }

            }, 
        }
    }

    fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
        if new_block.previous_hash != latest_block.hash {
            println!("Bock with id {} has wrong previous hash", new_block.id); 
            return false;
        } else if !new_block.hash.starts_with("0000") {
            println!("block with id: {} has invalid hash", new_block.id);  
            return false;    
        } else if new_block.id != latest_block.id +1 {
            println!("block with id {} is not the next block after the latest block with id: {}", 
            new_block.id, latest_block.id);
            return false;
        } else if digest(format!("{}{}{}{}{}", new_block.id, &new_block.previous_hash, &new_block.data, new_block.timestamp, new_block.nonce)) 
        != new_block.hash {
            println!("block with id {} has invalid hash", new_block.id); 
            return false; 
        }
        true
   }
    }

   impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now(); 
        let now_timestamp = now.timestamp(); 
        
        let (nonce, hash) = Block::mine_block(id, now_timestamp, &previous_hash, &data); 

        Self {
            id, 
            hash, 
            timestamp: now.timestamp(), 
            previous_hash, 
            data, 
            nonce
        }
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        println!("mining block ..."); 
        let mut nonce = 1; 

        loop {
            let block_string = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce); 
            let hash = digest(block_string); 
            if hash.starts_with("0000") {
                println!("mined! nonce: {}, hash: {}", nonce, hash);
                return (nonce, hash)
            }
            nonce += 1;
        }
    }
   }

use chrono::Utc;
use sha256::digest;
fn main () { 
    let mut new_BC = BlockChain::new(); 
    new_BC.starting_block();

    println!("{:?}", new_BC); 
   
    let new_block = Block::new(2, new_BC.blocks[0].hash.to_owned(), "Azam".to_string());
    new_BC.try_add_block(new_block);
}