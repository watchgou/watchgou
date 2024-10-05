use std::hash::{DefaultHasher, Hash, Hasher};

use base64::{engine::general_purpose::STANDARD, Engine};
use memchr::memmem;
use wasm_bindgen::prelude::wasm_bindgen;

///
/// 字符串计算hash
///
#[wasm_bindgen]
pub fn hash_64(c_str: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    c_str.hash(&mut hasher);
    hasher.finish()
}

///
/// import ./watchgou.js
/// encode_base64("watch dog");
///
#[wasm_bindgen]
pub fn encode_base64(bytes: &[u8]) -> String {
    // direct encode
    STANDARD.encode(bytes)
}

///
/// 检索字符串
///
#[wasm_bindgen]
pub fn macthing_str(keyword: &str, content: &str) -> Vec<usize> {
    let bytes = content.trim().as_bytes();
    let finder = memmem::Finder::new(keyword.trim().as_bytes());
    let mut iters = finder.find_iter(bytes);
    let mut vec: Vec<usize> = Vec::new();
    while let Some(x) = iters.next() {
        vec.push(x);
    }
    vec
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

    #[test]
    fn test_macthing_str() {
        let index = macthing_str("ll", "hello world poll");
        println!("index: {:#?}", index);
    }
}
