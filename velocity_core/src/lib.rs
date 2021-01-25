#![feature(num_as_ne_bytes)]
use std::{
    cmp::Ordering,
    sync::atomic::{AtomicU64, AtomicUsize},
    time::Instant,
};

use num_bigint::BigUint;
use num_traits::One;
use sha2::{Digest, Sha256};

pub const DIFFICULTY: usize = 4;
const MAX_NONCE: u64 = u64::MAX;
use rayon::prelude::*;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    /// Message content
    pub data: Vec<u8>,
    /// When it was sent
    pub timestamp: i64,
    /// Used for verification
    pub nonce: u64,
    /// Previous block
    pub prev_hash: Vec<u8>,
    /// Hash of the sender's public key
    pub from: Vec<u8>,
    /// Hash of the receiver's public key
    pub to: Vec<u8>,
}
impl Block {
    fn mine(&self) -> Option<u64> {
        let target = BigUint::one() << (256 - 4 * DIFFICULTY);
        println!("Target {}", target);
        let start = std::time::Instant::now();
        for nonce in 0..MAX_NONCE {
            let hash = self.calculate_hash(nonce);
            let hash_int = BigUint::from_bytes_be(&hash);
            print!("{:<15}\r", hash_int);
            if hash_int < target {
                let cracked = std::time::Instant::now();
                println!("\nCracked in {:?}", cracked - start);
                return Some(nonce);
            }
        }
        None
    }
    pub fn multithreaded_mine(&self) -> Option<u64> {
        let target = BigUint::one() << (256 - 4 * DIFFICULTY);
        // let handles = (0..num_cpus::get()).map(|i| {
        //     std::thread::spawn(|| {

        //     })
        // });
        let start = std::time::Instant::now();
        (0..MAX_NONCE).into_par_iter().find_map_any(|nonce| {
            let hash = self.calculate_hash(nonce);
            let hash_int = BigUint::from_bytes_be(&hash);
            print!("{:<15}\r", hash_int);
            if hash_int < target {
                let cracked = std::time::Instant::now();
                println!("\nCracked in {:?}", cracked - start);
                return Some(nonce);
            }
            None
        })
        // for i in (0..MAX_NONCE).into_par_iter().
        //     (0..MAX_NONCE)
        //         .into_par_iter()
        //         .map(|nonce| {
        //             if found.load(std::sync::atomic::Ordering::Relaxed) != 0{
        //                 return
        //             }
        //             let hash = self.calculate_hash(nonce);
        //             let hash_int = BigUint::from_bytes_be(&hash);
        //             print!("{:<15}\r", hash_int);
        //             if hash_int < target {
        //                 found.store(nonce, std::sync::atomic::Ordering::Relaxed);
        //                 // return Some(nonce);
        //             }
        //         })
        //         .collect::<()>();
        //         return found.load(std::sync::atomic::Ordering::Relaxed);
    }
   pub fn calculate_hash(&self, nonce: u64) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let mut headers = self.headers();
        headers.extend_from_slice(nonce.as_ne_bytes());
        hasher.update(headers);
        let finalized = hasher.finalize().as_slice().to_vec();

        finalized
    }
    fn headers(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(self.timestamp.as_ne_bytes());
        buffer.extend_from_slice(&self.prev_hash);

        buffer
    }
    pub fn hash(&self) -> Vec<u8> {
        self.calculate_hash(self.nonce)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}
impl Default for BlockChain {
    fn default() -> Self {
        Self::new()
    }
}
impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Self::genesis()],
        }
    }
    pub fn genesis() -> Block {
        Block {
            from: b"0".to_vec(),
            to: b"0".to_vec(),
            data: b"Hello Crypto".to_vec(),
            nonce: 0,
            prev_hash: b"".to_vec(),
            timestamp: chrono::Utc::now().timestamp_nanos(),
        }
    }
    pub fn add_to_chain(&mut self, data: &str, from: &str, to: &str) {
        let prev = self.blocks.last().unwrap();
        let hash = prev.hash();
        let mut block = Block {
            data: data.into(),
            from: from.into(),
            to: to.into(),
            nonce: 0,
            prev_hash: hash,
            timestamp: chrono::Utc::now().timestamp_nanos(),
        };
        let nonce = block.multithreaded_mine().unwrap();
        block.nonce = nonce;
        self.blocks.push(block);
    }
    pub fn save(&self) {
        std::fs::write("chain.bin", bincode::serialize(self).unwrap()).unwrap()
    }
    pub fn load() -> Self {
        println!("Loading");
        let content = std::fs::read("chain.bin").unwrap();
        println!("Loaded");
        bincode::deserialize(&content).unwrap()
    }
    // fn get_all_for_address(&self, address: Vec<u8>) -> Vec<&Block>{
    //     self.blocks.iter().filter(|b| {
    //         let data = b.data;

    //         true
    //     }).collect()
    // }
}

pub fn add_to_chain(amount: usize) {
    let mut chain = BlockChain::load();
    let f = vec![
        108, 95, 252, 40, 65, 174, 10, 51, 118, 37, 89, 129, 172, 27, 42, 120, 27, 32, 155, 183,
        41, 120, 159, 242, 214, 248, 136, 153, 245, 3, 163, 116,
    ];
    let from = unsafe { String::from_utf8_unchecked(f) };
    for _ in 0..amount {
        chain.add_to_chain(&fakedata_generator::gen_email(), "from", &from);
    }
    chain.save();
}
pub fn print_length() {
    let mut chain = BlockChain::load();
    println!("Loaded!");
    println!("Chain length: {}", chain.blocks.len());
    // chain.save();
}
pub fn repl() {
    println!("Loading chain...");
    let chain = BlockChain::load();
    println!("Chain loaded!");
    use std::io;
    use std::io::prelude::*;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line: String = line.unwrap();
        let start = Instant::now();
        let matches: Vec<&Block> = chain
            .blocks
            .iter()
            .filter(|b| {
                let s = String::from_utf8(b.data.clone()).unwrap();
                s.contains(&line)
            })
            .collect();
        let end = Instant::now();

        println!("Found {} matches in {:#?}", matches.len(), end - start);
    }
}
// #[test]
fn main() {
    let chain = BlockChain::new();
    chain.save();
    add_to_chain(20);
    // print_length();
    for block in &BlockChain::load().blocks {
        println!("Block from {:?}", block.from);
        println!("Block   to {:?}", block.to);
        println!("Block hash {:?}", block.hash());
        println!("===========");
    }
    panic!()
}
