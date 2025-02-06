build:
    cd worker && wasm-pack build --target web --out-dir ../pkg

up:
    miniserve .
