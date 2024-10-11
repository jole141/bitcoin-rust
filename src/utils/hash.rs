use secp256k1::hashes::{sha256, Hash};

pub fn sha256_hash(data: &str) -> sha256::Hash {
    sha256::Hash::hash(data.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let data = "Hello, World!";
        let expected_hash = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f";
        let hash = sha256_hash(data);
        assert_eq!(hash.to_string(), expected_hash);
    }
}