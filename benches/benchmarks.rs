use criterion::{criterion_group, criterion_main, Criterion, black_box};
use sha3::{Sha3_256, Digest};
use tiny_keccak::*;
use sha_3::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let text = b"33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";
    c.bench_function("My sha_3 implementation", |b| b.iter(|| {
        let hasher = SHA3_256::new();
        let hash = hasher.hash(black_box(text));
    }));
    c.bench_function("Crate sha3", |b| b.iter(|| {
        let mut hasher = Sha3_256::new();
        hasher.update(black_box(text));
        let hash = hasher.finalize();
    }));
    c.bench_function("Crate tiny_keccak", |b| b.iter(|| {
        let mut sha3 = Sha3::v256();
        sha3.update(black_box(text));
        let mut output = [0u8; 32];
        sha3.finalize(&mut output);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);