use clap::{App, Arg};
use crypto_cli::{bip32, keccak256, keccak256_selector};
use hex::encode;

// use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator}; // look into this

// look into derive

fn main() {
    let matches = App::new("Crypto CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Performs crypto operations")
        .arg(
            Arg::new("hash")
                .short('h')
                .long("hash")
                .value_name("STRING")
                .help("Hashes a string using keccak256")
                .takes_value(true),
        )
        .arg(
            Arg::new("mnemonic")
                .short('m')
                .long("mnemonic")
                .help("Generates a new mnemonic phrase and keys"),
        )
        .arg(
            Arg::new("selector")
                .short('s')
                .long("selector")
                .value_name("STRING")
                .help("Get a keccak function selector")
                .takes_value(true),
        )
        .get_matches();

    if let Some(input) = matches.value_of("hash") {
        let hash = keccak256(input);
        println!("Keccak256 hash: {}", hash);
    }

    if let Some(input) = matches.value_of("selector") {
        let selector = keccak256_selector(input);
        println!("Keccak256 function selector: {}", selector);
    }

    if matches.is_present("mnemonic") {
        let wallet = bip32::HDWallet::new();
        println!("Mnemonic: {}", wallet.mnemonic.phrase());

        // Show master extended keys
        println!("Master xprv: {}", wallet.get_extended_private_key("m"));
        println!("Master xpub: {}", wallet.get_extended_public_key("m"));

        // Derive Bitcoin key
        let btc_key = wallet.derive_key(bip32::CoinType::Bitcoin);
        println!("Bitcoin Private Key: {}", encode(btc_key));

        // Derive Ethereum key
        let eth_key = wallet.derive_key(bip32::CoinType::Ethereum);
        println!("Ethereum Private Key: {}", encode(eth_key));

        // Derive Solana key
        let sol_key = wallet.derive_key(bip32::CoinType::Solana);
        println!("Solana Private Key: {}", encode(sol_key));
    }
}
