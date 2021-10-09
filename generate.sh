#!/bin/sh

RUSTFLAGS='' wasm-pack build --target web --out-dir ../docs web

# python3 -m http.server
