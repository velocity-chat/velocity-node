use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    _ZERO,
    Init
}
impl Packet {
    pub fn toward_stream(&self) -> Vec<u8> {
        let mut ser = bincode::serialize(self).unwrap();
        let padding = (ser.len()..crate::PACKET_SIZE-ser.len()).map(|_| 0);
        ser.extend(padding.into_iter());
        ser
    }
}