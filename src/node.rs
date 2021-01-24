use std::{fmt::Debug, io::{Write}, net::{TcpListener, TcpStream, ToSocketAddrs}, sync::{Arc, Mutex}};

use crate::{connection::Connection, packet::Packet};
// pub type Connection = (TcpStream, SocketAddr);
pub type Peers = Arc<Mutex<Vec<Arc<Connection>>>>;
pub struct Node {
    peers: Arc<Mutex<Vec<Arc<Connection>>>>,
    listener: TcpListener,
    is_running: bool,
    debug_master: bool,
}
impl Node {
    pub fn create_server<A: ToSocketAddrs + Debug + Copy>(addr: A, master: bool) -> Self {
        let listener = TcpListener::bind(addr).expect(&format!("Failed to bind initial listening address. {:#?}", addr));
        Self {
            listener,
            peers: Arc::new(Mutex::new(Vec::new())),
            is_running: true,
            debug_master: master
        }
    }
    pub fn listen(&mut self) {
            if !self.debug_master {
                println!("Connecting to default peer on 2020");
                let mut peer = TcpStream::connect("127.0.0.1:2020").unwrap();
                peer.write(&Packet::Init.toward_stream()).unwrap();
                println!("Writing peer packet");
                self.peers.lock().unwrap().push(Arc::new(Connection {
                    addr: None,
                    stream: peer, // &peer, SocketAddr::from("127.0.0.1:2020")
                }));
            }

        while self.is_running {
            let mut handles = vec![];
            for (stream, addr) in self.listener.accept() {
                let apeers = Arc::clone(&self.peers);
                println!("Before pushing to handles");
                handles.push(std::thread::spawn(move || {
                    println!("New thread being spawned");
                    let connection = Connection {
                        stream,
                        addr: Some(addr),
                    };
                    let con = Arc::new(connection);
                    apeers.lock().unwrap().push(Arc::clone(&con));
                    match handle_connection(apeers, Arc::clone(&con)) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("A listening thread had an error and had to exit. {:#?}", e);
                        }
                    }
                }));
            }
            // for handle in handles {
            //     let joined = handle.join();
            //     let _ = joined.expect("A listening thread failed to join.");
            //     println!("Closed a thread down!");
            // }
        }
    }
}
pub fn handle_packet(pack: Packet, _apeers: Peers, _con: Arc<Connection>) {
    println!("Handling packet: {:#?}", pack);
}
pub fn handle_connection(
    apeers: Peers,
    con: Arc<Connection>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Handling connection");
    let stream = &con.stream;
    // stream.write(b"You connected\r\n")?;
    loop {
        // stream.write(b"You connected\r\n")?;
        // let buffer = &mut [0u8; crate::PACKET_SIZE];
        match bincode::deserialize_from::<&TcpStream, crate::packet::Packet>(stream){
            Ok(packet) => {
                if let Packet::_ZERO = packet {
                    continue;
                }
                handle_packet(packet, apeers.clone(), con.clone());
            }
            Err(err) => {
                println!("Malformed packet.");
                eprintln!("{}", err);
                // Err(err)
            }
        }

        // stream.read_exact(buffer)?;
        // handle_packet(buffer, Arc::clone(&apeers), Arc::clone(&con));
        // stream.write(buffer)?;
        // print!("Buffer read is {:#?}\r", buffer);
    }
    // Ok(())
}
