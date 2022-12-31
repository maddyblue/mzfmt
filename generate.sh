#!/bin/sh

# Need to:
# - comment out workspace-hack deps in the ore Cargo.toml
# - comment out workspace-hack deps in the sql-parser Cargo.toml
# - to ore's cast.rs, target32 mod, add: cast_from!(u64, usize);

PATH=$PATH:~/.cargo/bin

RUSTFLAGS='' wasm-pack build --target web --out-dir ../docs web

# python3 -m http.server
