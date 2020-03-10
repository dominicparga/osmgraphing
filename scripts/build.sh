#!/usr/bin/env sh

set -e

cargo fmt
cargo build
cargo build --release --features="osmgraphing"
