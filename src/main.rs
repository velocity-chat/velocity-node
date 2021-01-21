mod node;
mod connection;
fn main() {
    let mut server = node::Node::create_server("127.0.0.1:2021");
    server.listen()
}
