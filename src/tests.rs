#[cfg(test)]
mod test {
    use sha3::Digest;
    use hex::encode;
    
    #[test]
    fn sha3_224_test() {
        let text = b"Hello world!";

        let mut hasher_library_impl = sha3::Sha3_224::new();
        hasher_library_impl.update(text);
        let hash_library_impl = hasher_library_impl.finalize();

        let hasher_my_impl = sha_3::SHA3_224::new();
        let hash_my_impl = hasher_my_impl.hash(text);

        assert_eq!(encode(hash_library_impl), encode(hash_my_impl));
    }

    #[test]
    fn sha3_256_test() {
        let text = b"Twinkling Watermelon";

        let mut hasher_library_impl = sha3::Sha3_256::new();
        hasher_library_impl.update(text);
        let hash_library_impl = hasher_library_impl.finalize();

        let hasher_my_impl = sha_3::SHA3_256::new();
        let hash_my_impl = hasher_my_impl.hash(text);

        assert_eq!(encode(hash_library_impl), encode(hash_my_impl));
    }

    #[test]
    fn sha3_384_test() {
        let text = b"33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";

        let mut hasher_library_impl = sha3::Sha3_384::new();
        hasher_library_impl.update(text);
        let hash_library_impl = hasher_library_impl.finalize();

        let hasher_my_impl = sha_3::SHA3_384::new();
        let hash_my_impl = hasher_my_impl.hash(text);

        assert_eq!(encode(hash_library_impl), encode(hash_my_impl));
    }

    #[test]
    fn sha3_512_test() {
        let text = b"33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";

        let mut hasher_library_impl = sha3::Sha3_512::new();
        hasher_library_impl.update(text);
        let hash_library_impl = hasher_library_impl.finalize();

        let hasher_my_impl = sha_3::SHA3_512::new();
        let hash_my_impl = hasher_my_impl.hash(text);

        assert_eq!(encode(hash_library_impl), encode(hash_my_impl));
    }

    #[test]
    fn keccak_224_test() {
        let text = b"Hello world!";

        let mut hasher_library_impl = sha3::Keccak224::new();
        hasher_library_impl.update(text);
        let hash_library_impl = hasher_library_impl.finalize();

        let hasher_my_impl = sha_3::Keccak224::new();
        let hash_my_impl = hasher_my_impl.hash(text);

        assert_eq!(encode(hash_library_impl), encode(hash_my_impl));
    }

    #[test]
    fn keccak_256_test() {
        let text = b"Twinkling Watermelon";

        let mut hasher_library_impl = sha3::Keccak256::new();
        hasher_library_impl.update(text);
        let hash_library_impl = hasher_library_impl.finalize();

        let hasher_my_impl = sha_3::Keccak256::new();
        let hash_my_impl = hasher_my_impl.hash(text);

        assert_eq!(encode(hash_library_impl), encode(hash_my_impl));
    }

    #[test]
    fn keccak_384_test() {
        let text = b"33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";

        let mut hasher_library_impl = sha3::Keccak384::new();
        hasher_library_impl.update(text);
        let hash_library_impl = hasher_library_impl.finalize();

        let hasher_my_impl = sha_3::Keccak384::new();
        let hash_my_impl = hasher_my_impl.hash(text);

        assert_eq!(encode(hash_library_impl), encode(hash_my_impl));
    }

    #[test]
    fn keccak_512_test() {
        let text = b"33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";

        let mut hasher_library_impl = sha3::Keccak512::new();
        hasher_library_impl.update(text);
        let hash_library_impl = hasher_library_impl.finalize();

        let hasher_my_impl = sha_3::Keccak512::new();
        let hash_my_impl = hasher_my_impl.hash(text);

        assert_eq!(encode(hash_library_impl), encode(hash_my_impl));
    }
}