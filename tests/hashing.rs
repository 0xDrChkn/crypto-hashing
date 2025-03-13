#[cfg(test)]
mod tests {
    use crypto_cli::{keccak256, keccak256_selector}; // Move import inside the module

    #[test]
    fn keccak256_test() {
        let result = keccak256("InvalidSupplyAmount()");

        assert_eq!(
            result,
            "97fbc77c9bdf5d4df2b510b7fe05f55f27d66ae3d45c223dac498ff0b1c0f0b7"
        );
    }

    #[test]
    fn keccak256_selector_test() {
        let result = keccak256_selector("InvalidSupplyAmount()");
        assert_eq!(result, "0x97fbc77c");
    }

    #[test]
    fn test_function_signatures() {
        // Test common ERC20 function signatures
        assert_eq!(
            keccak256_selector("transfer(address,uint256)"),
            "0xa9059cbb"
        );
        assert_eq!(keccak256_selector("approve(address,uint256)"), "0x095ea7b3");
        assert_eq!(
            keccak256_selector("transferFrom(address,address,uint256)"),
            "0x23b872dd"
        );
    }

    #[test]
    fn test_event_topics() {
        // Test common Ethereum event signatures
        assert_eq!(
            keccak256("Transfer(address,address,uint256)"),
            "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        );
    }
}
