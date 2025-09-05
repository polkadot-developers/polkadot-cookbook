#!/bin/bash
set -e
rustup default 1.86
rustup target add wasm32-unknown-unknown --toolchain 1.86
rustup component add rust-src --toolchain 1.86
