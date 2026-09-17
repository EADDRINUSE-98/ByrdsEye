use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn compute_hash(path: &Path) -> io::Result<[u8; 32]> {
    let file_bytes = fs::read(path)?;

    let mut hasher = Sha256::new();

    hasher.update(&file_bytes);

    Ok(hasher.finalize())
}

// fn get_hash_status(path: &Path) {
//     match compute_hash(path) {
//         Ok(hash) => println!("{}\t{}", hash, file_name),
//         Err(e) => println!("Error: {}", e),
//     }
// }
