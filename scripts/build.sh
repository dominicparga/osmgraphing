#!/usr/bin/env sh

cargo fmt
cargo build
cargo build --release --features="osmgraphing"
