#!/bin/bash

source ./scripts/travis/helper.sh

#------------------------------------------------------------------------------#
# cargo build and test

cargo login "${CRATES_TOKEN}"
cargo publish
