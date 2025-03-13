use bip39::{Language, Mnemonic, MnemonicType, Seed};

pub struct Wallet {
    pub mnemonic: String,
}

impl Wallet {
    pub fn generate_mnemonic() -> String {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

        let phrase: &str = mnemonic.phrase();
        println!("phrase: {}", phrase);

        let seed = Seed::new(&mnemonic, "");

        // get the HD wallet seed as raw bytes
        let seed_bytes: &[u8] = seed.as_bytes();

        // print the HD wallet seed as a hex string
        println!("{:X}", seed);

        return phrase.to_string();
    }
}
