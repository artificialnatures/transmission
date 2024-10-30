use wasm_bindgen::prelude::*;

pub mod ecs;

#[wasm_bindgen]
pub fn read_transmission_message() -> String {
  "transmission".to_string()
}
