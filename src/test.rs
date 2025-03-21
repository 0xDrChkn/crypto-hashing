use bip39::{Language, Mnemonic, Seed};
use hex::encode;
use hmac::{Hmac, Mac};
use k256::{EncodedPoint, ecdsa::SigningKey};
use sha2::Sha512;

type HmacSha512 = Hmac<Sha512>;

/// Struct to store HD wallet information
pub struct HDWallet {
    pub mnemonic: Mnemonic,
    pub seed: Vec<u8>,
    pub master_private_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

impl HDWallet {
    /// Generate a new HD wallet (BIP-39 Mnemonic + BIP-32 Master Key)
    pub fn new() -> Self {
        let mnemonic = Mnemonic::random(&mut rand::thread_rng(), Language::English);
        let seed = Seed::new(&mnemonic, "").as_bytes().to_vec();

        let (master_private_key, chain_code) = HDWallet::generate_master_key(&seed);

        Self {
            mnemonic,
            seed,
            master_private_key,
            chain_code,
        }
    }

    /// BIP-32: Generate master private key & chain code from seed
    fn generate_master_key(seed: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut mac =
            HmacSha512::new_from_slice(b"Bitcoin seed").expect("HMAC can take key of any size");
        mac.update(seed);
        let result = mac.finalize().into_bytes();

        let master_private_key = result[..32].to_vec();
        let chain_code = result[32..].to_vec();

        (master_private_key, chain_code)
    }

    /// BIP-32: Derive a child private key
    pub fn derive_child_key(&self, index: u32) -> (Vec<u8>, Vec<u8>) {
        let mut mac =
            HmacSha512::new_from_slice(&self.chain_code).expect("HMAC can take key of any size");

        let mut data = vec![0u8];
        data.extend(&self.master_private_key);
        data.extend(&index.to_be_bytes());

        mac.update(&data);
        let result = mac.finalize().into_bytes();

        let child_private_key = result[..32].to_vec();
        let new_chain_code = result[32..].to_vec();

        (child_private_key, new_chain_code)
    }

    /// Convert a private key to a public key
    pub fn private_to_public(private_key: &[u8]) -> String {
        let signing_key = SigningKey::from_bytes(private_key.try_into().unwrap()).unwrap();
        let public_key = signing_key.verifying_key();
        encode(public_key.to_encoded_point(true).as_bytes()) // Compressed public key
    }
}

fn main() {
    let wallet = HDWallet::new();

    println!("Mnemonic Phrase: {}", wallet.mnemonic.to_string());
    println!("Seed: {}", encode(&wallet.seed));
    println!("Master Private Key: {}", encode(&wallet.master_private_key));
    println!("Master Chain Code: {}", encode(&wallet.chain_code));

    // Derive child key (e.g., m/0')
    let (child_key, child_chain_code) = wallet.derive_child_key(0);
    println!("Child Private Key: {}", encode(&child_key));
    println!("Child Chain Code: {}", encode(&child_chain_code));

    // Convert child private key to public key
    let child_public_key = HDWallet::private_to_public(&child_key);
    println!("Child Public Key: {}", child_public_key);
}
