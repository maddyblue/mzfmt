# mzfmt

SQL pretty printer for Materialize

## command line

To install to `.cargo/bin`, clone this repo and:

`cargo install --path .`

Then pipe a SQL file to `mzfmt`.

## web

https://mz.sqlfum.pt/ is a public site that compiles this crate to wasm.
It only needs a static web server (GitHub pages here) and runs entirely in the browser.
