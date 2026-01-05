#!/bin/bash

pushd klippa
wasm-pack build --scope nerd-bible --features wasm
pushd pkg
npm publish --access public
