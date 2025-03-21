use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::Network;
use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey};
use hex::encode;
use std::str::FromStr;

pub enum CoinType {
    Bitcoin,
    Ethereum,
    Solana,
}

pub struct HDWallet {
    pub mnemonic: Mnemonic,
    pub seed: Vec<u8>,
    pub master_key: ExtendedPrivKey,
}

impl HDWallet {
    /// Generate a new BIP-39 mnemonic and derive the BIP-32 master key
    pub fn new() -> Self {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let seed = Seed::new(&mnemonic, "").as_bytes().to_vec();

        // Create master key from seed
        let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
            .expect("Failed to derive master key");

        Self {
            mnemonic,
            seed,
            master_key,
        }
    }

    /// Create a wallet from an existing seed
    pub fn from_seed(seed_bytes: &[u8]) -> Self {
        // Create a dummy mnemonic (not used)
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let seed = seed_bytes.to_vec();

        // Create master key from seed
        let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
            .expect("Failed to derive master key");

        Self {
            mnemonic,
            seed,
            master_key,
        }
    }

    /// Get the BIP-44 derivation path for different blockchains
    fn get_derivation_path(coin: CoinType) -> &'static str {
        match coin {
            CoinType::Bitcoin => "m/44'/0'/0'/0/0",   // Bitcoin (BTC)
            CoinType::Ethereum => "m/44'/60'/0'/0/0", // Ethereum (ETH)
            CoinType::Solana => "m/44'/501'/0'/0/0",  // Solana (SOL)
        }
    }

    /// Derive a key for a specific blockchain
    pub fn derive_key(&self, coin: CoinType) -> [u8; 32] {
        let path_str = Self::get_derivation_path(coin);
        self.derive_key_from_path(path_str)
    }

    /// Derive a key from a specific path
    pub fn derive_key_from_path(&self, path_str: &str) -> [u8; 32] {
        let path = DerivationPath::from_str(path_str).expect("Invalid derivation path");

        let derived_key = self
            .master_key
            .derive_priv(&bitcoin::secp256k1::Secp256k1::new(), &path)
            .expect("Failed to derive child key");

        derived_key.private_key.secret_bytes()
    }

    /// Get the extended private key for a path
    pub fn get_extended_private_key(&self, path_str: &str) -> String {
        let path = DerivationPath::from_str(path_str).expect("Invalid derivation path");

        let derived_key = self
            .master_key
            .derive_priv(&bitcoin::secp256k1::Secp256k1::new(), &path)
            .expect("Failed to derive child key");

        derived_key.to_string()
    }

    /// Get the extended public key for a path
    pub fn get_extended_public_key(&self, path_str: &str) -> String {
        let path = DerivationPath::from_str(path_str).expect("Invalid derivation path");
        let secp = bitcoin::secp256k1::Secp256k1::new();

        let derived_key = self
            .master_key
            .derive_priv(&secp, &path)
            .expect("Failed to derive child key");

        let public_key = ExtendedPubKey::from_private(&secp, &derived_key);
        public_key.to_string()
    }
}
