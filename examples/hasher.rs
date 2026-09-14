use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io;

fn compute_hash(path: &str) -> io::Result<String> {
    // 4. Read entire file into memory.
    let file_bytes = fs::read(path)?;

    // 5. Init hasher
    let mut hasher = Sha256::new();

    // 6. Feed the entire file at once to hasher.
    hasher.update(&file_bytes);

    // 7. Finalize the hash
    let hash_result = hasher.finalize();

    // 8. Return Ok() variant
    Ok(hex::encode(hash_result))
}

fn main() {
    // 1. Collect all the argument passed to binary.
    let args: Vec<String> = env::args().collect();

    // 2. Get the path to the file.
    let file_name = &args[1];

    // 3. Get hash of the file.
    match compute_hash(file_name) {
        Ok(hash) => println!("{}\t{}", hash, file_name),
        Err(e) => println!("Error: {}", e),
    }
}
