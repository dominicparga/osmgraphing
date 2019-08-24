#!/bin/bash

# exit as soon as non-zero exit-code occurs
set -ev

#------------------------------------------------------------------------------#
# cargo build and test

cargo build --verbose --all
cargo test --verbose --all

#------------------------------------------------------------------------------#
# check version

echo 'TODO: check version'
echo "TRAVIS_TAG=${TRAVIS_TAG}"
export BLUB='omg'
