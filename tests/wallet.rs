#[cfg(test)]
mod tests {
    use crypto_cli::wallet::HDWallet;
    use hex::decode;

    #[test]
    fn test_fixed_seed_master_key() {
        // BIP-32 Test Vector 1: Seed (Hex)
        let seed = decode("000102030405060708090a0b0c0d0e0f").expect("Failed to decode hex seed");

        // Create wallet from seed
        let wallet = HDWallet::from_seed(&seed);

        // Check extended keys
        let master_xprv = wallet.get_extended_private_key("m");
        let master_xpub = wallet.get_extended_public_key("m");

        assert_eq!(
            master_xprv,
            "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"
        );
        assert_eq!(
            master_xpub,
            "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8"
        );
    }

    #[test]
    fn test_hardened_derivation() {
        let seed = decode("000102030405060708090a0b0c0d0e0f").expect("Failed to decode hex seed");
        let wallet = HDWallet::from_seed(&seed);

        // Test m/0'
        let child_xprv = wallet.get_extended_private_key("m/0'");
        let child_xpub = wallet.get_extended_public_key("m/0'");

        assert_eq!(
            child_xprv,
            "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7"
        );
        assert_eq!(
            child_xpub,
            "xpub68Gmy5EdvgibQVfPdqkBBCHxA5htiqg55crXYuXoQRKfDBFA1WEjWgP6LHhwBZeNK1VTsfTFUHCdrfp1bgwQ9xv5ski8PX9rL2dZXvgGDnw"
        );
    }

    #[test]
    fn test_deep_derivation() {
        let seed = decode("000102030405060708090a0b0c0d0e0f").expect("Failed to decode hex seed");
        let wallet = HDWallet::from_seed(&seed);

        // Test m/0'/1/2'/2/1000000000
        let deep_xprv = wallet.get_extended_private_key("m/0'/1/2'/2/1000000000");
        let deep_xpub = wallet.get_extended_public_key("m/0'/1/2'/2/1000000000");

        assert_eq!(
            deep_xprv,
            "xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76"
        );
        assert_eq!(
            deep_xpub,
            "xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy"
        );
    }
}
