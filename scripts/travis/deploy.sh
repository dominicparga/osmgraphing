#!/bin/bash

source ./scripts/travis/helper.sh

#------------------------------------------------------------------------------#
# deploy to cargo.io

cargo login "${CRATES_TOKEN}"
cargo publish
