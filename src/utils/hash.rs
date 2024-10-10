use secp256k1::hashes::{sha256, Hash};

pub fn sha256_hash(data: &str) -> sha256::Hash {
    sha256::Hash::hash(data.as_bytes())
}
