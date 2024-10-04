use std::hash::{DefaultHasher, Hash, Hasher};

use base64::{engine::general_purpose::STANDARD, Engine};
use wasm_bindgen::prelude::wasm_bindgen;

///
///     字符串计算hash
///
#[wasm_bindgen]
pub fn hash_64(c_str: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    c_str.hash(&mut hasher);
    hasher.finish()
}

///
///     import ./watchgou.js
///     encode_base64("watch dog");
///
#[wasm_bindgen]
pub fn encode_base64(bytes: &[u8]) -> String {
    // direct3D
    STANDARD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_base64() {
        let bytes = [121, 21, 23];
        let c_str = encode_base64(&bytes);
        println!("print: {:}", c_str);
    }

    #[test]
    fn test_hash_64() {
        let c_str = "hello wasm";
        let hash_value = hash_64(&c_str);
        println!("hash value: {}", hash_value);
    }
}
