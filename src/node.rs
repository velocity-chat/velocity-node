use std::{
    fmt::{write, Debug},
    io::Write,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use num_bigint::BigUint;
use num_traits::One;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use velocity_core::{add_to_chain, Block, BlockChain};

use crate::{connection::Connection, packet::Packet};
// pub type Connection = (TcpStream, SocketAddr);
pub type Peers = Arc<Mutex<Vec<Arc<Connection>>>>;
pub struct Node {
    peers: Peers,
    listener: TcpListener,
    is_running: bool,
    debug_master: bool,
    block: Arc<Mutex<BlockChain>>,
    handles: Vec<JoinHandle<()>>,
}
impl Node {
    pub fn create_server<A: ToSocketAddrs + Debug + Copy>(addr: A, is_debug_master: bool) -> Self {
        let listener = TcpListener::bind(addr).expect(&format!(
            "Failed to bind initial listening address. {:#?}",
            addr
        ));
        let blk = match is_debug_master {
            true => Arc::new(Mutex::new(BlockChain::load())),
            false => Arc::new(Mutex::new(BlockChain::new())),
        };
        Self {
            listener,
            peers: Arc::new(Mutex::new(Vec::new())),
            is_running: true,
            debug_master: is_debug_master,
            block: blk,
            handles: vec![],
        }
    }
    pub fn listen(&mut self) {
        if !self.debug_master {
            println!("Connecting to default peer on 2020");
            let mut peer = TcpStream::connect("127.0.0.1:2020").unwrap();
            // peer.write(&Packet::BlockUpdate(BlockChain::new()).toward_stream()).unwrap();
            peer.write(&Packet::Init.toward_stream()).unwrap();
            println!("Writing peer packet");

            let apeers = Arc::clone(&self.peers);
            let amchain = Arc::clone(&self.block);
            self.handles.push(std::thread::spawn(|| {
                let con = Arc::new(Connection {
                    addr: None,
                    stream: peer,
                });
                apeers.lock().unwrap().push(con.clone());

                match handle_connection(amchain, apeers, con.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("A listening thread had an error and had to exit. {:#?}", e);
                    }
                }
            }));
        }

        while self.is_running {
            // let mut handles = vec![];
            // let cloned = self.peers.clone();
            // let p = cloned.lock().unwrap();
            // for peer in p.iter() {
            //     let amchain        = Arc::clone(&self.block);
            //     let apeers = Arc::clone(&self.peers);
            //     handles.push(std::thread::spawn(|| {
            //         match handle_connection(amchain, apeers, peer.clone()) {
            //             Ok(_) => {}
            //             Err(e) => {
            //                 println!("A listening thread had an error and had to exit. {:#?}", e);
            //             }
            //         }
            //     }));
            // }
            for (stream, addr) in self.listener.accept() {
                let apeers = Arc::clone(&self.peers);
                let amchain = Arc::clone(&self.block);

                println!("Before pushing to handles");
                self.handles.push(std::thread::spawn(move || {
                    println!("New thread being spawned");
                    let connection = Connection {
                        stream,
                        addr: Some(addr),
                    };
                    let con = Arc::new(connection);
                    apeers.lock().unwrap().push(Arc::clone(&con));
                    match handle_connection(amchain, apeers, Arc::clone(&con)) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("A listening thread had an error and had to exit. {:#?}", e);
                        }
                    }
                }));
            }
            //TODO: should I join handles? probably not
            // for handle in handles {
            //     let joined = handle.join();
            //     let _ = joined.expect("A listening thread failed to join.");
            //     println!("Closed a thread down!");
            // }
        }
    }
}
pub fn handle_packet(
    pack: Packet,
    chain: Arc<Mutex<BlockChain>>,
    apeers: Peers,
    con: Arc<Connection>,
) {
    println!("<- {:?}", pack);
    match pack {
        Packet::_ZERO => {}
        Packet::Init => {}
        // Packet::BlockUpdate(ref blk) => {
        //     println!("New block! Length {}", blk.blocks.len());
        //     drop(apeers.lock().unwrap().clone().iter_mut().map(|f| {
        //         let fc = f.clone();
        //         let mut s = &fc.stream;
        //         s.write(&pack.toward_stream()).unwrap();
        //         println!("Sending bloclupdate to a peer");
        //     }).collect::<Vec<_>>());
        //     *chain.lock().unwrap() = blk.to_owned();
        //     chain.lock().unwrap().save();
        // }
        Packet::NewBlock(b) => {
            let solution = b.multithreaded_mine();
            if solution.is_some() {
                println!("sending solutioN!");
                (&con.stream)
                    .write(&Packet::Solution(b, solution.unwrap()).toward_stream())
                    .unwrap();
            }
            // let temp_chain = chain.clone().lock().unwrap();
            // temp_chain.add_to_chain();
            // let solution = b
        }
        Packet::Solution(b, nonce) => {
            let target = BigUint::one() << (256 - 4 * velocity_core::DIFFICULTY);
            let hash = b.calculate_hash(nonce);
            let hash_int = BigUint::from_bytes_be(&hash);
            if hash_int < target {
                chain.lock().unwrap().blocks.push(b);
                // (&con.stream)
                // .write(&Packet::AcceptedSolution.toward_stream())
                // .unwrap();
            } else {
                (&con.stream)
                    .write(&Packet::RejectedSolution.toward_stream())
                    .unwrap();
            }
        }
        Packet::AcceptedSolution => {
            println!("Mining solution accepted! Yay!")
        }
        Packet::RejectedSolution => {
            println!("Mining solution denied! F!")
        }
    }
    // println!("Handling packet: {:#?}", pack);
}
pub fn handle_connection(
    chain: Arc<Mutex<BlockChain>>,
    apeers: Peers,
    con: Arc<Connection>,
) -> Result<(), Box<dyn std::error::Error>> {
    let stream = &con.stream;
    let b = Block {
        data: b"Hello world".to_vec(),
        from: b"From".to_vec(),
        to: b"To".to_vec(),
        nonce: 0,
        prev_hash: chain.lock().unwrap().blocks.last().unwrap().hash(),
        timestamp: chrono::Utc::now().timestamp_nanos(),
    };
    
    if unsafe { crate::SEND_BLOCK } {
        (&con.stream)
            .write(&Packet::NewBlock(b).toward_stream())
            .unwrap();
        println!("Sent newblock packet");
        println!("{}", apeers.lock().unwrap().len());
    }

    // (&con.stream).write(&Packet::Init.toward_stream()).unwrap();
    // chain.lock().unwrap().add_to_chain("zz", "from", "to");
    // println!("Sending blockupdate");
    // stream.write(&Packet::BlockUpdate(chain.lock().unwrap().clone()).toward_stream())?;
    loop {
        match bincode::deserialize_from::<&_, crate::packet::Packet>(stream) {
            Ok(packet) => {
                if let Packet::_ZERO = packet {
                    continue;
                }
                handle_packet(packet, chain.clone(), apeers.clone(), con.clone());
            }
            Err(err) => {
                eprintln!("Malformed packet. Breaking. {}", err);
                break Err(err);
            }
        }
    }
}
