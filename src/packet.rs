use serde::{Deserialize, Serialize};
use velocity_core::{Block, BlockChain};
#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    _ZERO,
    Init,
    NewBlock(Block),
    Solution(Block, u64),
    AcceptedSolution,
    RejectedSolution
    // BlockUpdate(BlockChain)
}
impl Packet {
    pub fn toward_stream(&self) -> Vec<u8> {
        println!("-> {:?}", self);
        let mut ser = bincode::serialize(self).unwrap();
        let padding = (ser.len()..crate::PACKET_SIZE-ser.len()).map(|_| 0);
        ser.extend(padding.into_iter());
        ser
    }
}