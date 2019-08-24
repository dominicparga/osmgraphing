#!/bin/bash

# exit as soon as non-zero exit-code occurs
set -ev

#------------------------------------------------------------------------------#
# run all cargo-tests

cargo test --verbose --all
