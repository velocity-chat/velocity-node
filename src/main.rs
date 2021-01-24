
mod node;
mod connection;
mod packet;
pub const PACKET_SIZE: usize = 2048;
fn main() {
    let mut is_default_node = false;
    for arg in std::env::args() {
        if arg == "--master" {
            is_default_node = true;
            println!("Default Node.")
        }
    }
    let a = if is_default_node { "127.0.0.1:2020"} else { "127.0.0.1:2021"};
    println!("Will bind on {},", a);
    let mut server = node::Node::create_server(a, is_default_node);
    server.listen()
}
