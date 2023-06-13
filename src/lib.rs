pub use ::anyhow;
pub use ::js_sys;
pub use ::serde_wasm_bindgen;
pub use ::wasm_bindgen;
use js_sys::JsString;
pub use rs2js_macro::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub trait Rs2JsObj {
    fn to_js(&self) -> JsValue;
    fn from_js(js: JsValue) -> anyhow::Result<Self>
    where
        Self: Sized;
}

/// Custom bindings to avoid using fallible `Reflect` for plain objects.
#[wasm_bindgen]
extern "C" {
    pub type ObjectExt;

    #[wasm_bindgen(method, indexing_getter)]
    pub fn get_with_ref_key(this: &ObjectExt, key: &JsString) -> JsValue;

    #[wasm_bindgen(method, indexing_setter)]
    pub fn set(this: &ObjectExt, key: JsString, value: JsValue);
}
