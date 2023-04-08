#!/bin/sh

set -ex

npm i ../webpack --force
cd ../vscode-web-wasm-webpack-plugin
npm i
npm run build
cd ../vscode-rust-web
npm i
npm run compile-web
