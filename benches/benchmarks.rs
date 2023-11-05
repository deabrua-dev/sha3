use criterion::{criterion_group, criterion_main, Criterion};
use sha3::{Sha3_256, Digest};
use sha_3::*;
use tiny_keccak::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let text = b"Hello world!";
    c.bench_function("My sha_3 implementation", |b| b.iter(|| {
        keccak(text, SHA3::SHA3_256).unwrap();
    }));
    c.bench_function("Crate sha3", |b| b.iter(|| {
        let mut hasher = Sha3_256::new();
        hasher.update(text);
        let hash = hasher.finalize();
    }));
    c.bench_function("Crate tiny_keccak", |b| b.iter(|| {
        let mut sha3 = Sha3::v256();
        sha3.update(text);
        let mut output = [0u8; 32];
        sha3.finalize(&mut output);
    }));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
