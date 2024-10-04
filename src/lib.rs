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
///     import ./watchgou
///     encode_base64();
///
#[wasm_bindgen]
pub fn encode_base64(bytes: &[u8]) -> String {
    // direct3D
    STANDARD.encode(bytes)
}
