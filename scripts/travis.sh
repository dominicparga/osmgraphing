#!/bin/bash

_usage="
USAGE
    ${0} [-h --help] [build | test | deploy]*

DESCRIPTION
       Wrapper for quick interaction with this project.
       Every option is meant to be used in the CI/CD-pipeline, thus they are
       verbose.

       -h --help
              Prints this help message and exits.

       build
              Builds via cargo and checks for matching tag-versions between git
              and cargo.

       test
              Executes the defined test-cases.

       deploy
              Deploys to https://crates.io/crates/osmgraphing
"

#------------------------------------------------------------------------------#
# general setup

# exit as soon as non-zero exit-code occurs
set -ev

# colors
_nc='\033[0m'
_black='\033[0;30m'
_red='\033[0;31m'
_green='\033[0;32m'
_brown='\033[0;33m'
_blue='\033[0;34m'
_purple='\033[0;35m'
_cyan='\033[0;36m'
_light_gray='\033[0;37m'
_dark_gray='\033[1;30m'
_light_red='\033[1;31m'
_light_green='\033[1;32m'
_yellow='\033[1;33m'
_light_blue='\033[1;34m'
_light_purple='\033[1;35m'
_light_cyan='\033[1;36m'
_white='\033[1;37m'

#------------------------------------------------------------------------------#
# helper functions

__help() {
    echo -e "${_yellow}${_usage}${_nc}"
}

__build() {
    #--------------------------------------------------------------------------#
    # check version

    # if tag is provided
    # -> deploy later
    # -> check versions before everything to save runtime
    if [[ -n "${TRAVIS_TAG}" ]]; then
        _osmgraphing_version="v$(cat ./Cargo.toml | grep 'version = ".*"' | sed 's_.*version.*"\(.*\)".*_\1_')"
        if [[ "${TRAVIS_TAG}" != "${_osmgraphing_version}" ]]; then
            echo -e "${_red}Error: The version '${_osmgraphing_version}' in 'Cargo.toml' doesn't match the provided tag '${TRAVIS_TAG}'.${_nc}"
            _errcode=1
            return
        fi
    fi

    #--------------------------------------------------------------------------#
    # cargo build

    cargo build --verbose --all

    #--------------------------------------------------------------------------#
    # check deployment

    if [[ -n "${TRAVIS_TAG}" ]]; then
        cargo publish --dry-run
    fi
}

__test() {
    cargo test --verbose --all
}

__deploy() {
    if [[ -z "${CRATES_TOKEN}" ]]; then
        echo -e "${_red}Error: \${CRATES_TOKEN} is zero.${_nc}"
        _errcode=1
        return
    fi

    cargo doc
    # deploy to cargo.io
    cargo publish --token "${CRATES_TOKEN}"
}

#------------------------------------------------------------------------------#
# cmdline parser

# no args -> print usage and exit
if [[ "${#}" -eq 0 ]]; then
    __help
    exit 0
fi

while [[ "${#}" -gt 0 ]]; do
    case "${1}" in
    -h|--help)
        shift
        _errcode=0
        ;;
    build)
        shift
        __build
        ;;
    test)
        shift
        __test
        ;;
    deploy)
        shift
        __deploy
        ;;
    *)
        echo -e "${_red}Error: unknown argument '${1}'.${_nc}"
        echo
        _errcode=1
    esac

    if [[ -n "${_errcode}" ]]; then
        __help
        exit ${_errcode}
    fi
done
