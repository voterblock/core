use std::iter::zip;
use std::str::FromStr;
use crate::types::{Ballot, Block};
use crate::util::crypto_util::sha256;

#[derive(Debug)]
pub struct Blockchain {
  pub blocks: Vec<Block>,
}

impl Blockchain {

  pub fn new(ballot: &str) -> Self {
    return Blockchain {
      blocks: vec![Self::create_genesis_block(ballot.to_string())]
    };
  }

  pub fn get_ballot(&self) -> Ballot {
    return Ballot::from_str(self.get_genesis_block().data.as_str()).unwrap();
  }

  pub fn hash(data: &String, prev_hash: &String, ballot_hash: &String, timestamp: &u128, sequence: &u128) -> String {
    let data_hash = sha256(data);
    let timestamp_hash = sha256(&timestamp.to_string());
    let sequence_hash = sha256(&sequence.to_string());
    return sha256(&format!("{}{}{}{}{}", data_hash, prev_hash, ballot_hash, timestamp_hash, sequence_hash));
  }

  pub fn add_block(&mut self, block: Block) {
    if block.prev_hash == self.get_last_hash()
      && block.sequence == self.get_last_sequence() + 1
      && block.hash == Blockchain::hash(&block.data, &block.prev_hash, &block.ballot_hash, &block.timestamp, &block.sequence)
      && block.ballot_hash == self.get_genesis_block().ballot_hash
      && self.ballot_questions_answered(&block)
    {
      self.blocks.push(block);
    }
  }

  pub fn get_last_sequence(&self) -> u128 {
    return self.blocks[self.blocks.len() - 1].sequence;
  }

  pub fn get_last_hash(&self) -> String {
    return match self.blocks.len() {
      0 => String::from("Genesis"),
      n => self.blocks[n - 1].hash.clone()
    }
  }

  pub fn get_genesis_block(&self) -> &Block {
    return &self.blocks[0];
  }

  fn ballot_questions_answered(&self, block: &Block) -> bool {
    let ballot = self.get_ballot();
    let answers = block.data.lines().map(String::from).collect::<Vec<String>>();
    return ballot.questions.len() == answers.len()
      && zip(ballot.questions, answers).all(|(question, answer)| question.choices.contains(&answer))
  }

  fn create_genesis_block(ballot: String) -> Block {
    let ballot_hash = sha256(ballot.as_str());
    return Block::new(ballot, String::from(""), ballot_hash, 0);
  }
}