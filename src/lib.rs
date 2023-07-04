use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(left: usize, right: usize) {
    alert(&format!("{} + {} = {}", left, right, left + right));
}
