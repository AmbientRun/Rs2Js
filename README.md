# Rs2Js: Rust to JsValue and back again

## Usage

```rust
#[derive(Rs2Js, Debug, Clone, PartialEq)]
pub struct DbProject {
    pub name: String,
    pub owner_id: String,
    pub created: Timestamp,
}
fn main() {
    let proj = DbProject { .. };
    let value = proj.to_js(); // JsValue
}
```

## Motivation

This mostly exists because [serde-wasm-bindgen](https://github.com/cloudflare/serde-wasm-bindgen) currently doesn't support passing through JsValues ([see this issue](https://github.com/cloudflare/serde-wasm-bindgen/issues/32)).
