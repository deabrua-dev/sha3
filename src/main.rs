mod tests;
mod sha_3;

use std::time::Instant;
use crate::sha_3::*;

fn main() {
    let text = b"Hello world!";
    let start = Instant::now();
    let result = keccak(text, SHA3::SHA3_512).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed in sha_3 is: {:?}", duration.as_nanos());
}