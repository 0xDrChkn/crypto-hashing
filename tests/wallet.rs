#[cfg(test)]
mod tests {
    use crypto_cli::Wallet; // Import Wallet from the parent module

    #[test]
    fn test_generate_mnemonic() {
        let mnemonic = Wallet::generate_mnemonic();

        // Check that the mnemonic consists of exactly 12 words
        assert_eq!(mnemonic.split_whitespace().count(), 12);
    }
}
