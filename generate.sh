#!/bin/sh

RUSTFLAGS='' wasm-pack build --target web --out-dir ../docs web
