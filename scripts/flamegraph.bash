#!/usr/bin/env bash

set -e

# build
cur_dir="$(dirname "$(pwd)"/"${0}")"
"${cur_dir}/build.sh"

# cargo install flamegraph, see https://github.com/killercup/cargo-flamegraph
cargo flamegraph --bin osmgraphing -- --config "${cur_dir}/../custom/config.yaml"
