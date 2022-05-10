use std::collections::{HashMap, HashSet};
use std::iter::{zip};
use std::str::FromStr;
use openssl::pkey::PKey;
use crate::types::{Ballot, Block};
use crate::util::crypto_util::{sha256, validate_signature};

#[derive(Debug)]
pub struct Blockchain {
  pub blocks: Vec<Block>,
  pub registered_voters: HashSet<String>
}

impl Blockchain {

  pub fn new(ballot: &str) -> Self {
    return Blockchain {
      blocks: vec![Self::create_genesis_block(ballot.to_string())],
      registered_voters: HashSet::new()
    };
  }

  pub fn register_voter(&mut self, public_key: String) {
    self.registered_voters.insert(public_key);
  }

  pub fn get_ballot(&self) -> Ballot {
    return Ballot::from_str(self.get_genesis_block().data.as_str()).unwrap();
  }

  pub fn hash(signature: &String, public_key: &String, data: &String, prev_hash: &String, ballot_hash: &String, timestamp: &u128, sequence: &u128) -> String {
    let sig_hash = sha256(signature);
    let key_hash = sha256(public_key);
    let data_hash = sha256(data);
    let timestamp_hash = sha256(&timestamp.to_string());
    let sequence_hash = sha256(&sequence.to_string());
    return sha256(&format!("{}{}{}{}{}{}{}", sig_hash, key_hash, data_hash, prev_hash, ballot_hash, timestamp_hash, sequence_hash));
  }

  pub fn add_block(&mut self, block: Block) {
    if block.prev_hash == self.get_last_hash()
      && block.sequence == self.get_last_sequence() + 1
      && block.hash == Blockchain::hash(&block.signature, &block.public_key, &block.data, &block.prev_hash, &block.ballot_hash, &block.timestamp, &block.sequence)
      && block.ballot_hash == self.get_genesis_block().ballot_hash
      && self.registered_voters.contains(&block.public_key)
      && self.signature_valid(&block.signature, &block.public_key)
      && self.has_not_voted(&block.public_key)
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

  pub fn tally_votes(&self) -> HashMap<String, HashMap<String, u128>> {
    let questions = self.collect_ballot_questions();
    let answers = self.collect_ballot_answers();
    return zip(questions, answers).collect();
  }

  fn signature_valid(&self, signature: &String, public_key: &String) -> bool {
    let signature_bytes = hex::decode(signature).unwrap();
    let public_key_bytes = base64::decode(public_key).unwrap();
    let public_key = PKey::public_key_from_pem(&public_key_bytes).unwrap();

    return match validate_signature(public_key, &public_key_bytes, &signature_bytes) {
      Ok(true) => true,
      _ => false
    };
  }

  fn has_not_voted(&self, public_key: &String) -> bool {
    let voters: Vec<String> = self.blocks.iter().skip(1).map(|block| block.public_key.clone()).collect();
    return !voters.contains(public_key);
  }

  fn ballot_questions_answered(&self, block: &Block) -> bool {
    let ballot = self.get_ballot();
    let answers = block.data.lines().map(String::from).collect::<Vec<String>>();
    return ballot.questions.len() == answers.len()
      && zip(ballot.questions, answers).all(|(question, answer)| question.choices.contains(&answer))
  }

  fn create_genesis_block(ballot: String) -> Block {
    let ballot_hash = sha256(ballot.as_str());
    return Block::new(String::from(""), String::from(""), ballot, String::from(""), ballot_hash, 0);
  }

  fn collect_ballot_questions(&self) -> Vec<String> {
    return self.get_ballot().questions.iter().map(|q| q.question.clone()).collect();
  }

  fn collect_ballot_answers(&self) -> Vec<HashMap<String, u128>> {
    return self.blocks.iter().skip(1)
      .fold(vec![], |mut tally, block | {
        for (question_index, answer) in block.data.lines().enumerate() {
          match tally.get_mut(question_index) {
            Some(question_tally) => {
              question_tally
                  .entry(answer.to_string())
                  .and_modify(|count| *count += 1)
                  .or_insert(1);
            },
            None => {
              tally.push(HashMap::from([(answer.to_string(), 1)]));
            }
          }
        }
        return tally;
      });
  }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
  use openssl::pkey::{PKey};
  use crate::types::{Block, Blockchain};
  use crate::util::crypto_util::{generate_key_pair, rsa_sign, sha256, validate_signature};

  fn test_ballot() -> String {
    return String::from("\
      \"Do you want to allow surfing prior to 6pm?\"::[Yes][No][Maybe So]\n\
      \"Who do you want for president\"::[Donald Trump][Joe Biden][Bernie Bro]\n\
    ");
  }

  fn test_block(blockchain: &Blockchain) -> Block {
    let keypair = generate_key_pair();
    let public_key_bytes = keypair.public_key_to_pem().unwrap();
    let public_key_pem = String::from_utf8(public_key_bytes.clone()).unwrap();
    let public_key = PKey::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
    let signature = rsa_sign(keypair, public_key_bytes.as_slice());

    assert_eq!(validate_signature(public_key, public_key_bytes.as_slice(), signature.as_slice()).unwrap(), true);

    return Block::new(
      hex::encode(signature),
      base64::encode(public_key_pem),
      String::from("Yes\nBernie Bro"),
      blockchain.get_last_hash(),
      sha256(test_ballot().as_str()),
      blockchain.get_last_sequence());
  }

  #[test]
  fn add_block() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());
    let block = test_block(&blockchain);

    blockchain.register_voter(block.public_key.clone());
    blockchain.add_block(block.clone());

    assert_eq!(blockchain.blocks.len(), 2);
    assert_eq!(blockchain.blocks.contains(&block), true);
  }

  #[test]
  fn add_block__prev_hash_validated() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());
    let mut block = test_block(&blockchain);
    block.prev_hash = String::from("foobar");
    block.hash();

    blockchain.register_voter(block.public_key.clone());
    blockchain.add_block(block.clone());

    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks.contains(&block), false);
  }

  #[test]
  fn add_block__sequence_validated() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());
    let mut block = test_block(&blockchain);
    block.sequence = 3;
    block.hash();

    blockchain.register_voter(block.public_key.clone());
    blockchain.add_block(block.clone());

    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks.contains(&block), false);
  }

  #[test]
  fn add_block__hash_validated() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());
    let mut block = test_block(&blockchain);
    block.hash = String::from("foobar");

    blockchain.register_voter(block.public_key.clone());
    blockchain.add_block(block.clone());

    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks.contains(&block), false);
  }

  #[test]
  fn add_block__ballot_hash_validated() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());
    let mut block = test_block(&blockchain);
    block.ballot_hash = String::from("foobar");
    block.hash();

    blockchain.register_voter(block.public_key.clone());
    blockchain.add_block(block.clone());

    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks.contains(&block), false);
  }

  #[test]
  fn add_block__voter_registration_validated() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());
    let block = test_block(&blockchain);

    blockchain.add_block(block.clone());

    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks.contains(&block), false);
  }

  #[test]
  fn add_block__signature_validated() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());
    let mut block = test_block(&blockchain);
    block.signature = hex::encode("foobar");
    block.hash();

    blockchain.register_voter(block.public_key.clone());
    blockchain.add_block(block.clone());

    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks.contains(&block), false);
  }

  #[test]
  fn add_block__vote_count_validated() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());

    let block_one = test_block(&blockchain);
    blockchain.register_voter(block_one.public_key.clone());
    blockchain.add_block(block_one.clone());

    assert_eq!(blockchain.blocks.len(), 2);
    assert_eq!(blockchain.blocks.contains(&block_one), true);

    let mut block_two = test_block(&blockchain);
    block_two.public_key = block_one.public_key.clone();
    block_two.signature = block_one.signature.clone();
    block_two.hash();

    blockchain.add_block(block_two.clone());

    assert_eq!(blockchain.blocks.len(), 2);
    assert_eq!(blockchain.blocks.contains(&block_one), true);
    assert_eq!(blockchain.blocks.contains(&block_two), false);
  }

  #[test]
  fn add_block__incomplete_ballot_validated() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());
    let mut block = test_block(&blockchain);
    block.data = String::from("Yes");
    block.hash();

    blockchain.register_voter(block.public_key.clone());
    blockchain.add_block(block.clone());

    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks.contains(&block), false);
  }

  #[test]
  fn add_block__invalid_ballot_answers_validated() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());
    let mut block = test_block(&blockchain);
    block.data = String::from("foo\nbar");
    block.hash();

    blockchain.register_voter(block.public_key.clone());
    blockchain.add_block(block.clone());

    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks.contains(&block), false);
  }

  #[test]
  fn add_block__tally_votes() {
    let mut blockchain = Blockchain::new(test_ballot().as_str());

    let block_one = test_block(&blockchain);
    blockchain.register_voter(block_one.public_key.clone());
    blockchain.add_block(block_one.clone());

    let block_two = test_block(&blockchain);
    blockchain.register_voter(block_two.public_key.clone());
    blockchain.add_block(block_two.clone());

    let votes = blockchain.tally_votes();

    assert_eq!(blockchain.blocks.len(), 3);
    assert_eq!(blockchain.blocks.contains(&block_one), true);
    assert_eq!(blockchain.blocks.contains(&block_two), true);
    assert_eq!(votes["Do you want to allow surfing prior to 6pm?"]["Yes"], 2);
    assert_eq!(votes["Who do you want for president"]["Bernie Bro"], 2);
  }
}