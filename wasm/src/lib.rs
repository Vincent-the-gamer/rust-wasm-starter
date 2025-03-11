use demo::add_two;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn add(left: u8, right: u8) -> u8 {
    add_two(left, right)
}