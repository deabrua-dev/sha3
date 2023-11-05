mod tests;

use sha_3::*;

fn main() {
    let text = b"Twinkling Watermelon";
    let precompiled_sha3_output = "e9419f6cd10ee5fb4ee12468f94613b8c25bb75acea2ac8b913c324b6bbd4db3";

    let mut result = [0; 32];
    
    let hasher = SHA3_256::new();
    hasher.hash(text, &mut result);

    println!("{}", hex::encode(result));
    if precompiled_sha3_output == hex::encode(result) {
        println!("Yes!");
    }
}