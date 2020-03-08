#!/usr/bin/env sh

clear
cargo fmt
cargo build
cargo build --release --features="osmgraphing"
