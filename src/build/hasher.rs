use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;

// Function to compute SHA-256 hash of a file
pub fn compute_hash(file_path: &str) -> String {
    let mut file =
        fs::File::open(file_path).expect("Failed to open file for hashing");
    let mut sha256 = Sha256::new();
    let mut buffer = [0; 1024]; // Buffer size can be adjusted
    loop {
        let n = file
            .read(&mut buffer)
            .expect("Failed to read file for hashing");
        if n == 0 {
            break;
        }
        sha256.update(&buffer[..n]);
    }
    format!("{:x}", sha256.finalize())
}
