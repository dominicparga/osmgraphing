#!/bin/bash

# exit as soon as non-zero exit-code occurs
set -ev

#------------------------------------------------------------------------------#
# check version

# if tag is provided
# -> deploy later
# -> check versions before everything to save runtime
if [[ -n "${TRAVIS_TAG}" ]]; then
    OSMGRAPHING_VERSION="v$(cat ./Cargo.toml | grep 'version' | sed 's_.*version.*"\(.*\)".*_\1_')"
    if [[ "${TRAVIS_TAG}" != "${OSMGRAPHING_VERSION}" ]]; then
        echo "The version in 'Cargo.toml' doesn't match the provided tag '${TRAVIS_TAG}'."
        exit 1
    fi
fi

#------------------------------------------------------------------------------#
# cargo build

cargo build --verbose --all

#------------------------------------------------------------------------------#
# check deployment

if [[ -n "${TRAVIS_TAG}" ]]; then
    cargo publish --dry-run
fi
