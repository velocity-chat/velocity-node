use chrono::prelude::*;
use num_bigint::BigUint;
use num_traits::One;
use sha2::{Sha256, Digest};
const HASH_BYTE_SIZE: usize = 8;
const DIFFICULTY: usize = 5;
pub type Sha256Hash = [u8; HASH_BYTE_SIZE];
const MAX_NONCE: u64 = 1_000_000;

struct Block {
    timestamp: i64,
    prev_block_hash: Sha256Hash,
    nonce: u64,
    data: Vec<u8>,
}

impl Block {
    pub fn genesis() -> Option<Self> {
        Self::new("Genesis block", Sha256Hash::default())
    }
    pub fn new(data: &str, prev_hash: Sha256Hash) -> Option<Self> {
        let mut s = Self {
            prev_block_hash: prev_hash,
            data: data.to_owned().into(),
            timestamp: Utc::now().timestamp(),
            nonce: 0,
        };
        s.try_hash().and_then(|nonce| {
            s.nonce = nonce;
            Some(s)
        })
        // s
    }
    fn try_hash(&self) -> Option<u64> {
        let target = BigUint::one() << (256 - 4 * DIFFICULTY);
        for nonce in 0..MAX_NONCE {
            
            let hash = Self::calculate_hash(&self, nonce);
            let hash_int = BigUint::from_bytes_be(&hash);

            if hash_int < target {
                return Some(nonce);
            }
        }

        None
    }

    pub fn calculate_hash(block: &Block, nonce: u64) -> Sha256Hash {
        let mut headers = block.headers();
        headers.extend_from_slice(&convert_u64_to_u8_array(nonce));

        let mut hasher = Sha256::new();
        hasher.update(&headers);
        let mut hash = Sha256Hash::default();

        let finalized = hasher.finalize();

        hasher
    }

    pub fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend(&convert_u64_to_u8_array(self.timestamp as u64));
        vec.extend_from_slice(&self.prev_block_hash);

        vec
    }
}



pub fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
    return [
        val as u8,
        (val >> 8) as u8,
        (val >> 16) as u8,
        (val >> 24) as u8,
        (val >> 32) as u8,
        (val >> 40) as u8,
        (val >> 48) as u8,
        (val >> 56) as u8,
    ];
}
struct BlockChain {
    blocks: Vec<Block>
}
impl BlockChain {
    pub fn new() -> Option<Self> {
        let blocks = Block::genesis()?;
        Some(Self {
            blocks: vec![blocks]
        })
    }
     // Adds a newly-mined block to the chain.
     pub fn add_block(&mut self, data: &str) -> Option<()> {
        let block: Block;
        {
            match self.blocks.last() {
                Some(prev) => {
                    let prevhash = prev.try_hash()?;
                    let prevhash = convert_u64_to_u8_array(prevhash);
                    block = Block::new(data, prevhash)?;
                }
                // Adding a block to an empty blockchain is an error, a genesis block needs to be
                // created first.
                None => {
                    return None
                }
            }
        }

        self.blocks.push(block);

        Some(())
    }

    // A method that iterates over the blockchain's blocks and prints out information for each.
    pub fn traverse(&self) {
        for (i, block) in self.blocks.iter().enumerate() {
            println!("block: {}", i);
            println!("Hash {:#?}", block.prev_block_hash);
            // println!("hash: {:?}", block.pretty_hash());
            // println!("parent: {:?}", block.pretty_parent());
            // println!("data: {:?}", block.pretty_data());
            println!()
        }
    }
}
fn main() {
    // let b = BlockChain::new().unwrap();
    let mut chain = BlockChain::new().unwrap();
    println!("Send 1 RC to foo");
    chain.add_block("cool block bro!").unwrap();
    chain.add_block("cool block bro!").unwrap();
    chain.add_block("cool block bro!").unwrap();
    chain.add_block("cool block bro!").unwrap();

    println!("Traversing blockchain:\n");
    chain.traverse();

}