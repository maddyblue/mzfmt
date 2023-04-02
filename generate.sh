#!/bin/sh

export RUSTFLAGS=''
wasm-pack build --target web --release --out-dir ../docs web
wasm-pack build web

# python3 -m http.server
