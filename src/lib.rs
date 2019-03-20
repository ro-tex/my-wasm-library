use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]

pub fn excited_greeting(original: &str) -> String {
    format!("HELLO {}", original.to_uppercase())
}
