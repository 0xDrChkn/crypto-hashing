use tiny_keccak::{Hasher, Keccak};

fn keccak256_hash(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

pub fn keccak256(data: &str) -> String {
    keccak256_hash(data.as_bytes())
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn keccak256_selector(data: &str) -> String {
    let hash: String = keccak256_hash(data.as_bytes())
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    format!("0x{}", hash[..8].to_string())
}
