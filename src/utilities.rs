use sha3::{Digest, Sha3_256};



/// Hash a byte array using SHA3-256 and return the hash as a hex-encoded string (lowercase).
pub fn digest(bytes: &[u8]) -> String {
    hex::encode(sha3_256_hash(bytes))
}

/// Hash a byte array using the SHA3-256 (kekkac) hashing algorithm.
pub fn sha3_256_hash(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    result[..].to_vec()
}

