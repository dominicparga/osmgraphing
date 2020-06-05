#!/usr/bin/env bash

set -e
cur_dir="$(dirname "$(pwd)"/"${0}")"

# build
cargo build --release

# cargo install flamegraph, see https://github.com/killercup/cargo-flamegraph
cargo flamegraph --bin osmgraphing -- --config "${cur_dir}/../custom/config.yaml"
