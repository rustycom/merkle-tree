use sha3::{Digest, Sha3_256};

pub fn hash_string_sha3(input: &String) -> String {
    // Create a new SHA-3 hasher
    let mut hasher = Sha3_256::new();
    // Update the hasher with the input string
    hasher.update(input.as_bytes());
    // Finalize the hash and obtain the result as a fixed-size array
    let result = hasher.finalize();
    // Convert the result to a hexadecimal string
    let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
    hex_string
}