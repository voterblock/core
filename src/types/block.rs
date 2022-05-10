use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::Blockchain;

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
  pub hash: String,
  pub prev_hash: String,
  pub ballot_hash: String,
  pub signature: String,
  pub public_key: String,
  pub data: String,
  pub timestamp: u128,
  pub sequence: u128,
}

impl Block {

  pub fn new(signature: String, public_key: String, data: String, prev_hash: String, ballot_hash: String, prev_sequence: u128) -> Self {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let sequence = prev_sequence + 1;
    let hash = Blockchain::hash(&signature, &public_key, &data, &prev_hash, &ballot_hash, &timestamp, &sequence);
    return Block { hash, prev_hash, ballot_hash, signature, public_key, data, timestamp, sequence };
  }

  pub fn hash(&mut self) {
    self.hash = Blockchain::hash(&self.signature, &self.public_key, &self.data, &self.prev_hash, &self.ballot_hash, &self.timestamp, &self.sequence);
  }
}