#!/bin/sh

export RUSTFLAGS=''

# for mac:
export AR=/opt/homebrew/opt/llvm/bin/llvm-ar
export CC=/opt/homebrew/opt/llvm/bin/clang

wasm-pack build --target web --release --out-dir ../docs web
wasm-pack build web

# python3 -m http.server
