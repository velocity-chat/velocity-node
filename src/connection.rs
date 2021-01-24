use std::{io::Write, net::{SocketAddr, TcpStream}};
#[derive(Debug)]
pub struct Connection {
    pub stream: TcpStream,
    pub addr: Option<SocketAddr>,
}
impl Write for Connection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}