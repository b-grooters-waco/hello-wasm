# Hello, WASM

A simple WASM module to experiment with basic WASM concepts.

## Building

### WASM Pack
You will need to ensure that WASM pack for Rust is installed on your machine. You can do this with cargo:

```bash
cargo install wasm-pack
```

You can build normally with cargo, but to build the WASM web target you will need to use wasm-pack:

```bash
wasm-pack build --target web --out-dir web/pkg
```

wasm-pack 
* compiles your Rust code to WebAssembly
* runs wasm-bindgen to generate a JavaScript wrapper
* creates a pkg directory and moves the JavaScript wrapper to this location
* produces a package.json 
* copies the README.md into the package