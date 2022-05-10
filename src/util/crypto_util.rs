use openssl::error::ErrorStack;
use sha2::{Digest, Sha256};
use openssl::sign::{Signer, Verifier};
use openssl::rsa::Rsa;
use openssl::pkey::{PKey, Private, Public};
use openssl::hash::MessageDigest;

pub fn sha256(data: &str) -> String {
  return format!("{:x}", Sha256::digest(format!("{}", data)));
}

pub fn generate_key_pair() -> PKey<Private> {
  let rsa = Rsa::generate(2048).unwrap();
  return PKey::from_rsa(rsa).unwrap();
}

pub fn rsa_sign(private_key: PKey<Private>, data: &[u8]) -> Vec<u8> {
  let mut signer = Signer::new(MessageDigest::sha256(), &private_key).unwrap();
  signer.update(data).unwrap();
  return signer.sign_to_vec().unwrap();
}

pub fn validate_signature(public_key: PKey<Public>, data: &[u8], signature: &[u8]) -> Result<bool, ErrorStack> {
  // Verify the data
  let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key).unwrap();
  verifier.update(data).unwrap();
  return verifier.verify(&signature);
}