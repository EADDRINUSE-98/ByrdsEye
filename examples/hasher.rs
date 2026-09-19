use sha2::{Digest, Sha256};
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

// fn compute_hash(path: &Path) -> io::Result<[u8; 32]> {
fn compute_hash(path: &String) -> io::Result<[u8; 32]> {
    // Open a file.
    let mut file = File::open(path)?;
    // Init a sha256 hasher.
    let mut hasher = Sha256::new();
    // init a 4KB buffer.
    // let mut buffer = [0u8; 4096];

    // init a 8KB buffer.
    // let mut buffer = [0u8; 8192];

    // init a 64KB buffer.
    // let mut buffer = [0u8; 65536];

    // init a 128KB buffer.
    // let mut buffer = [0u8; 131072];

    // init a 256KB buffer.
    let mut buffer = [0u8; 262144];

    // Infinite loop
    loop {
        // Read from `file` to fill the `buffer` completely. Also capture the number of bytes read.
        let mut bytes_read = file.read(&mut buffer)?;
        println!("Bytes read: {}", bytes_read);

        // Update the hash incrementally. Buffer needs to read to the number of bytes that are
        // actucally read. `buffer[..bytes_read]` is used instead of `buffer` because when the last
        // chunk read, it might not fill the buffer 4KB entirely. So if I use `buffer` then the
        // entire buffer will be read and some part of previous chunk will be read as the part of new chunk,
        // which leads to incorrect hash.
        hasher.update(&buffer[..bytes_read]);
        // Stop the loop if nothing is read.
        if bytes_read == 0 {
            println!("Stop reading!");
            break;
        }
    }

    Ok(hasher.finalize().into())
}

// fn get_hash(path: &Path) {
//     match compute_hash(path) {
//         Ok(hash) => println!("{}\t{}", hash, path),
//         Err(e) => println!("Error: {}", e),
//     }
// }
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    match compute_hash(path) {
        Ok(hash) => println!("{}\t{}", hex::encode(hash), path),
        Err(e) => println!("Error: {}", e),
    }
}

// Test Results:
// For 4KB buffer:
//732377e7f4a2abdc13ddfa1eb4c9c497fd2a2b294674d056cf51581b47dd586d        /tmp/test_file
// real    3m26.065s
// user    2m57.437s
// sys     0m23.958s
//
// For 8KB buffer:
//732377e7f4a2abdc13ddfa1eb4c9c497fd2a2b294674d056cf51581b47dd586d        /tmp/test_file
// real    2m52.115s
// user    2m40.823s
// sys     0m11.022s
//
// For 64KB buffer:
//732377e7f4a2abdc13ddfa1eb4c9c497fd2a2b294674d056cf51581b47dd586d        /tmp/test_file
// real    2m0.902s
// user    1m55.392s
// sys     0m5.413s
//
// For 128KB buffer:
//732377e7f4a2abdc13ddfa1eb4c9c497fd2a2b294674d056cf51581b47dd586d        /tmp/test_file
// real    1m56.452s
// user    1m51.129s
// sys     0m5.204s
//
// For 256KB buffer:
