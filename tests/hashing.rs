#[cfg(test)]
mod tests {
    use crypto_hashing::{keccak256, keccak256_selector}; // Move import inside the module

    #[test]
    fn keccak256_test() {
        let result = keccak256("InvalidSupplyAmount()");
   
        assert_eq!(result, "97fbc77c9bdf5d4df2b510b7fe05f55f27d66ae3d45c223dac498ff0b1c0f0b7");
    }

    #[test]
    fn keccak256_selector_test() {
        let result = keccak256_selector("InvalidSupplyAmount()");
        assert_eq!(result, "0x97fbc77c");
    }
}

