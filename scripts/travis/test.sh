#!/bin/bash

source ./scripts/travis/helper.sh

#------------------------------------------------------------------------------#
# run all cargo-tests

cargo test --verbose --all
