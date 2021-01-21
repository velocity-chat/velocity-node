use std::{
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    sync::{Arc, Mutex},
};
pub type Connection = (TcpStream, SocketAddr);

pub struct Node {
    peers: Arc<Mutex<Vec<Connection>>>,
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

                handles.push(std::thread::spawn(move || {
                    apeers.lock().unwrap().push((stream, addr));
                    for peer in apeers.lock().unwrap().iter() {
                        println!("Peer! {:#?}", peer);
                    }
                    // self.handle_connection((stream, addr))
                }));
            }
            for handle in handles {
                let joined = handle.join();
                let _ = joined.expect("A listening thread failed to join.");
            }
        }
    }
    pub fn handle_connection(&mut self, con: Connection) {}
}
