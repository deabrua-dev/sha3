#[cfg(test)]
mod test {
    use sha_3::*;
    use hex::encode;
    
    #[test]
    fn sha3_224_test() {
        let text = b"Hello world!";
        let precompiled_sha3_output = "d3ee9b1ba1990fecfd794d2f30e0207aaa7be5d37d463073096d86f8";

        let mut result = [0; 28];

        let hasher = SHA3_224::new();
        hasher.hash(text, &mut result);

        assert_eq!(precompiled_sha3_output, encode(result));
    }

    #[test]
    fn sha3_256_test() {
        let text = b"Twinkling Watermelon";
        let precompiled_sha3_output = "e9419f6cd10ee5fb4ee12468f94613b8c25bb75acea2ac8b913c324b6bbd4db3";

        let mut result = [0; 32];

        let hasher = SHA3_256::new();
        hasher.hash(text, &mut result);

        assert_eq!(precompiled_sha3_output, encode(result));
    }

    #[test]
    fn sha3_384_test() {
        let text = b"33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";
        let precompiled_sha3_output = "12bd50d9444609624b6bff13140b0c477a7a45300691744cc7f6af8ebbf66404f22c19c21c1432438f26a5ec5d044a9a";

        let mut result = [0; 48];

        let hasher = SHA3_384::new();
        hasher.hash(text, &mut result);

        assert_eq!(precompiled_sha3_output, encode(result));
    }

    #[test]
    fn sha3_512_test() {
        let text = b"33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";
        let precompiled_sha3_output = "d86f5dd6ce4c0f4e6a8010a35f9f8bb8db3e4db578897146a07b24f7c4ab267b704a2f37b7043680c26be8168419a91c926f41e5cbb63eb639d3fcd15cdf90a3";

        let mut result = [0; 64];

        let hasher = SHA3_512::new();
        hasher.hash(text, &mut result);

        assert_eq!(precompiled_sha3_output, encode(result));
    }

    #[test]
    fn keccak_224_test() {
        let text = b"Hello world!";
        let precompiled_keccak_output = "2528dc7b1a2c2c45d31ba2e5b21d46f78558160ef046a04df20e2233";

        let mut result = [0; 28];

        let hasher = Keccak224::new();
        hasher.hash(text, &mut result);

        assert_eq!(precompiled_keccak_output, encode(result));
    }

    #[test]
    fn keccak_256_test() {
        let text = b"Twinkling Watermelon";
        let precompiled_keccak_output = "e853c0d2c020b013bfff1d214238e2d0d9f96fed3aad12a7a65a15028662c653";

        let mut result = [0; 32];

        let hasher = Keccak256::new();
        hasher.hash(text, &mut result);

        assert_eq!(precompiled_keccak_output, encode(result));
    }

    #[test]
    fn keccak_384_test() {
        let text = b"33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";
        let precompiled_keccak_output = "68ca5f8547d34f5435abf048c70e49c4e7d4bea205d345599c673bd4239681834aec9f3edb6fe3962b400796eb2c28db";

        let mut result = [0; 48];

        let hasher = Keccak384::new();
        hasher.hash(text, &mut result);

        assert_eq!(precompiled_keccak_output, encode(result));
    }

    #[test]
    fn keccak_512_test() {
        let text = b"33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";
        let precompiled_keccak_output = "60e91fad62a9492da46b5916d19d2251491600d0e37a3aaa6ecc4b2e582b2b07d42fc4d235be9adcaaad5c2489b9b7d6eec9dad14d54967d43d832d30ada15ed";

        let mut result = [0; 64];

        let hasher = Keccak512::new();
        hasher.hash(text, &mut result);

        assert_eq!(precompiled_keccak_output, encode(result));
    }
}