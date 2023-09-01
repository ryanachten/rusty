# Rusty

## Hello world

### What's inside

- `wasm-bindgen` is used to interface with JavaScript

```rust
#[wasm_bindgen]
extern {
    fn alert(s: &str); // import the window.alert JavaScript function
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!"); // call the JavaScript function
}
```

### Building the project

- Build WebAssembly via `wasm-pack build`
- Produces artifacts in a `pkg/*` folder
- the `.js` file produced by `wasm-bindgen` contains glue for importing DOM and JS functions
- it also produces TS definitions to consume in TypeScript

### Putting it into a web page

- We can use our pakcage in the web via a `create-wasm-app` JS project template
  - run `npm init wasm-app www`
- Creates an NPM project under the `www` subdirectory

### Using the local `wasm-game-of-life` package in `www`

- We can consume the local `wasm-game-of-life` package instead of via NPM we can reference it by `file`

```json
  "dependencies": {
    "wasm-game-of-life": "file:../pkg"
  },
```

- when updating the WASM file, we might need to run `npm update` / `yarn upgrade` to force the NPM module to update to use the latest local file
