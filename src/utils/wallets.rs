use secp256k1::ecdsa::Signature;
use secp256k1::hashes::Hash;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

use super::hash::sha256_hash;


pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    (secret_key, public_key)
}

pub fn sign_with_key(message: &str, secret_key: &SecretKey) -> Signature {
    let secp = Secp256k1::new();
    let digest = sha256_hash(message);
    let message = Message::from_digest(digest.to_byte_array());
    secp.sign_ecdsa(&message, secret_key)
}

pub fn verify_signature(message: &str, signature: &Signature, public_key: &PublicKey) -> bool {
    let secp = Secp256k1::new();
    let digest = sha256_hash(message);
    let message = Message::from_digest(digest.to_byte_array());
    secp.verify_ecdsa(&message, signature, public_key).is_ok()
}
