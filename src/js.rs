use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/split.es.js")]
extern "C" {
    #[wasm_bindgen(js_name = default)]
    pub type Split;

    #[wasm_bindgen(constructor, js_class = default)]
    pub fn new(idsOption: js_sys::Array, options: js_sys::Object) -> Split;

    #[wasm_bindgen(method, js_class = default)]
    pub fn destroy(this: &Split, preserveStyles: js_sys::Boolean, preserveGutter: js_sys::Boolean);

    #[wasm_bindgen(method, js_class = default, js_name = "getSizes")]
    pub fn get_sizes(this: &Split) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = default, js_name = "setSizes")]
    pub fn set_sizes(this: &Split, new_sizes: js_sys::Array);

    #[wasm_bindgen(method, js_class = default)]
    pub fn collapse(this: &Split, i: usize);
}
