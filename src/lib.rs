use std::hash::{DefaultHasher, Hash, Hasher};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hash_64(c_str: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    c_str.hash(&mut hasher);
    hasher.finish()
}
