use voter_block_core::types::{Block, Blockchain};
use voter_block_core::util::crypto_util::sha256;

fn main() {
    let ballot = "\
        \"Do you want to allow surfing prior to 6pm?\"::[Yes][No][Maybe So]\n\
        \"Who do you want for president\"::[Donald Trump][Joe Biden][Bernie Bro]\n\
        ";

    let mut blockchain = Blockchain::new(ballot);
    for _ in 1..5 {
        let block = Block::new(
            String::from("Yes\nBernie Bro"),
            blockchain.get_last_hash(),
            sha256(ballot),
            blockchain.get_last_sequence());

        blockchain.add_block(block);
    }
    println!("{:#?}", blockchain);
}
