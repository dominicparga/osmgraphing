#!/usr/bin/env bash

set -e

# build
cur_dir="$(dirname "$(pwd)"/"${0}")"
"${cur_dir}/build.sh"

# test
cargo test
cargo run --example dijkstra
cargo run --example parser
