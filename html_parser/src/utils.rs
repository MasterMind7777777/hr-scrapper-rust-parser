use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn compute_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);
    let hash_result = hasher.result_str();
    hash_result
}

