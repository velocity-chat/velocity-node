use std::{fmt::write, fs::read, io::{Read, Write}, net::{TcpListener, ToSocketAddrs}, sync::{Arc, Mutex, MutexGuard}};

use crate::connection::Connection;
// pub type Connection = (TcpStream, SocketAddr);
pub type Peers = Arc<Mutex<Vec<Arc<Connection>>>>;
pub struct Node {
    peers: Arc<Mutex<Vec<Arc<Connection>>>>,
    listener: TcpListener,
    is_running: bool,
}
impl Node {
    pub fn create_server<A: ToSocketAddrs>(addr: A) -> Self {
        let listener = TcpListener::bind(addr).expect("Failed to bind initial listening address.");
        Self {
            listener,
            peers: Arc::new(Mutex::new(Vec::new())),
            is_running: true,
        }
    }
    pub fn listen(&mut self) {
        while self.is_running {
            let mut handles = vec![];
            for (stream, addr) in self.listener.accept() {
                let apeers = Arc::clone(&self.peers);
                println!("Before pushing to handles");
                handles.push(std::thread::spawn(move || {
                    println!("New thread being spawned");
                    let connection = Connection { stream, addr };
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
pub fn handle_connection(
    mut apeers: Peers,
    con: Arc<Connection>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Handling connection");
    
    loop {
        // let mut buffer = Vec::new();
        let mut stream = &con.stream;
        stream.write(b"Jello")?;
        // stream.read(&mut buffer)?;
        // stream.write(&buffer)?;
        // println!("Buffer read is {:#?}", buffer);
    }
    Ok(())
}
