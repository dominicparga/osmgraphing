#!/bin/bash

source ./scripts/travis/helper.sh

#------------------------------------------------------------------------------#
# check version

# if tag is provided
# -> deploy later
# -> check versions before everything to save runtime
if [[ -n "${TRAVIS_TAG}" ]]; then
    OSMGRAPHING_VERSION="v$(cat ./Cargo.toml | grep 'version' | sed 's_.*version.*"\(.*\)".*_\1_')"
    if [[ "${TRAVIS_TAG}" != "${OSMGRAPHING_VERSION}" ]]; then
        echo "${RED}The version in 'Cargo.toml' doesn't match the provided tag '${TRAVIS_TAG}'.${NC}"
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
