use clap::{App, Arg};
use crypto_cli::{Wallet, keccak256, keccak256_selector};

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
                .help("Generates a new mnemonic phrase"),
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
        let mnemonic = Wallet::generate_mnemonic();
        println!("Generated mnemonic: {}", mnemonic);
    }
}
