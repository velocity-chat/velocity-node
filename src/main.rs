
mod node;
mod connection;
mod packet;
pub const PACKET_SIZE: usize = 2048 * 1024;
pub static mut SEND_BLOCK: bool = false;
//TODO: use an env variable
fn main() {
    let mut is_default_node = false;
    for arg in std::env::args() {
        if arg == "--master" {
            is_default_node = true;
            println!("Default Node.")
        }
    }
    if !is_default_node {
        unsafe {
            SEND_BLOCK = true;
        }
    }
    let binding_address = if is_default_node { "127.0.0.1:2020"} else { "127.0.0.1:2021"};
    let mut server = node::Node::create_server(binding_address, is_default_node);
    server.listen()
}
