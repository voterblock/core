use voter_block_core::types::{Block, Blockchain};
use voter_block_core::util::crypto_util::{sha256};

fn main() {
    let ballot = "\
        \"Do you want to allow surfing prior to 6pm?\"::[Yes][No][Maybe So]\n\
        \"Who do you want for president\"::[Donald Trump][Joe Biden][Bernie Bro]\n\
        ";

    let mut blockchain = Blockchain::new(ballot);
    blockchain.register_voter(String::from("LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FROEFNSUlCQ2dLQ0FRRUFsWGdKb1gzL2o0L1VCVmczd2tUTApDazA2OTJRY0M0cmc3N2lwaWE4ZkNCZVU1dHN3VTZValdPazNiVjRZQ3ZUQUU3R0VoUTB3MlliMWVmK1NKcXl5CjRmdzJyMXZxa3M2bWdESXVRVnlYMitPeXZSaStSVGFzdkdxTnFtbEFrWWQrTEM1OGY2QVlBWVJHNDBQeDVCbGsKd1lmZUpTd1pEcjlUOE53WkVWYk05OU9uelBUc3lHOHhYTDE2Z0pVbk5uZjNOV1d2VnYwL084L3N6VXc2QkdybAo5VDRKaGtwbnI3L0VMZ1Y4Yy9sRkYvUmRpM2N0K0YydWxIMG9UVHhsWkY5UGpBRGM2YkY1eUgxZGZmT2NPQzFqCnFVVThnUUxUZGg2RjhTaXRnbE5kNVpaMW5Oa04rcXpxTkVoRldTRCsxWG0zT1pxU2lqZXZTMm1sKzJRQzdMQ1cKSVFJREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg=="));
    blockchain.add_block(Block::new(
        String::from("7f0a101b64a3c2ca8f44eed77fa5ccf77beccbc5d4142329d7745582a3676a03ae0368134cfe6dee32afbf52d4ad2cbe54c52a0c947317c1dfda80b6937b7e88eda01f6fd99b75f59f86deab6124e5f36e5bdcb1b9d5ed9a75d12d79f98fcaf31441a50a6e6fd097853aaefea530d0bbc1e827027aacd471446abc5bad5766cade3274cb8a2aabbc0dbc33205acf97bcc73d72c485cfa8b79080a7a71b657a20c72f76499d92d8ffd904e7a681d19934786fbf75ab5d7e666429a3c6fe345e18aeaebe710065791001db3ce4d2ade7114977c83a5c9393afaf3d6e3db39a9c177ec0ba9b40a2fdc9c80be934e89a11e5363030f4157a0c54ff7371221ae8b4fc"),
        String::from("LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FROEFNSUlCQ2dLQ0FRRUFsWGdKb1gzL2o0L1VCVmczd2tUTApDazA2OTJRY0M0cmc3N2lwaWE4ZkNCZVU1dHN3VTZValdPazNiVjRZQ3ZUQUU3R0VoUTB3MlliMWVmK1NKcXl5CjRmdzJyMXZxa3M2bWdESXVRVnlYMitPeXZSaStSVGFzdkdxTnFtbEFrWWQrTEM1OGY2QVlBWVJHNDBQeDVCbGsKd1lmZUpTd1pEcjlUOE53WkVWYk05OU9uelBUc3lHOHhYTDE2Z0pVbk5uZjNOV1d2VnYwL084L3N6VXc2QkdybAo5VDRKaGtwbnI3L0VMZ1Y4Yy9sRkYvUmRpM2N0K0YydWxIMG9UVHhsWkY5UGpBRGM2YkY1eUgxZGZmT2NPQzFqCnFVVThnUUxUZGg2RjhTaXRnbE5kNVpaMW5Oa04rcXpxTkVoRldTRCsxWG0zT1pxU2lqZXZTMm1sKzJRQzdMQ1cKSVFJREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg=="),
        String::from("Yes\nBernie Bro"),
        blockchain.get_last_hash(),
        sha256(ballot),
        blockchain.get_last_sequence()));

    let votes = blockchain.tally_votes();
    println!("{:#?}", blockchain);
    println!("{:#?}", votes);
}
