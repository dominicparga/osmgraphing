#!/usr/bin/env sh

set -e

cargo fmt -- --check

cargo build
cargo build --release --features="osmgraphing"
