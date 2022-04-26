use sha2::{Digest, Sha256};

pub fn sha256(data: &str) -> String {
  return format!("{:x}", Sha256::digest(format!("{}", data)));
}