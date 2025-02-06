## Prerequisites to use
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)
- The Rust programming language
- `miniserve` (install with `cargo install miniserve`)
- Optionally, [the `just` command runner](https://github.com/casey/just) to use the justfile (`cargo install just`)

## How to use
- Use `just build` to build the WASM package.
- Use `miniserve .` in the project root to run the file.
- Visit `localhost:8080/index.html`, paste in your API key and click submit!

## Notes
The prompt is hard coded to just ask "Who are you?" to OpenAI on GPT-4O. However, you can adjust this however you want.

The caveat is basically that the worker crate needs to compile to WASM. The entire `rig-core` crate compiles to WASM. However, if you plan on using the companion crates they are likely to not compile to WASM, as they are not intended to be used in a webpage WASM context.
