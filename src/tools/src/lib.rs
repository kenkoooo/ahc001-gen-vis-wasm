mod gen;
mod utils;
mod vis;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate(seed: String) -> String {
    if let Ok(seed) = seed.parse::<u64>() {
        gen::gen(seed)
    } else {
        format!("Cannot parse seed: {}", seed)
    }
}

#[wasm_bindgen]
pub fn visualize(input_str: String, output_str: String) -> String {
    let (_, svg) = vis::visualize(&input_str, &output_str);
    svg
}

#[wasm_bindgen]
pub fn calc_score(input_str: String, output_str: String) -> String {
    let (score, _) = vis::visualize(&input_str, &output_str);
    score.to_string()
}
