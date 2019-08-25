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
set -e

#------------------------------------------------------------------------------#
# colors

_c_nc='\033[0m'
_c_black='\033[0;30m'
_c_red='\033[0;31m'
_c_green='\033[0;32m'
_c_brown='\033[0;33m'
_c_blue='\033[0;34m'
_c_purple='\033[0;35m'
_c_cyan='\033[0;36m'
_c_light_gray='\033[0;37m'
_c_dark_gray='\033[1;30m'
_c_light_red='\033[1;31m'
_c_light_green='\033[1;32m'
_c_yellow='\033[1;33m'
_c_light_blue='\033[1;34m'
_c_light_purple='\033[1;35m'
_c_light_cyan='\033[1;36m'
_c_white='\033[1;37m'

_c_info="${_c_cyan}"
_c_warn="${_c_yellow}"
_c_error="${_c_red}"
_c_nice="${_c_green}"

#------------------------------------------------------------------------------#
# helper functions

__help() {
    echo -e "${_c_info}${_usage}${_c_nc}"
}

__build() {
    echo -e "${_c_info}Starting${_c_nc} build phase"

    #--------------------------------------------------------------------------#
    # check version

    # if tag is provided
    # -> deploy later
    # -> check versions before everything to save runtime
    if [[ -n "${TRAVIS_TAG}" ]]; then
        echo -e "${_c_info}Checking${_c_nc} tag and version"
        _osmgraphing_version="v$(cat ./Cargo.toml | grep 'version = ".*"' | sed 's_.*version.*"\(.*\)".*_\1_')"
        if [[ "${TRAVIS_TAG}" != "${_osmgraphing_version}" ]]; then
            echo -e "${_c_error}ERROR:${_c_nc} The version ${_c_info}${_osmgraphing_version}${_c_nc} in 'Cargo.toml' doesn't match the provided tag ${_c_info}${TRAVIS_TAG}${_c_nc}."
            _errcode=1
            return
        fi
    fi

    #--------------------------------------------------------------------------#
    # cargo build

    echo -e "${_c_info}Starting${_c_nc} build"
    cargo --color always build --verbose --all

    #--------------------------------------------------------------------------#
    # check deployment

    if [[ -n "${TRAVIS_TAG}" ]]; then
        echo -e "${_c_info}Starting${_c_nc} dry-publish"
        cargo --color always publish --dry-run
    fi
}

__test() {
    echo -e "${_c_info}Starting${_c_nc} test phase"
    cargo --color always test --verbose --all
}

__deploy() {
    echo -e "${_c_info}Starting${_c_nc} deploy to cargo.io"

    if [[ -z "${CRATES_TOKEN}" ]]; then
        echo -e "${_c_error}ERROR:${_c_nc} \${CRATES_TOKEN} is zero."
        _errcode=1
        return
    fi

    cargo --color always doc
    # deploy to cargo.io
    cargo --color always publish --token "${CRATES_TOKEN}"
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
        echo -e "${_c_error}ERROR:${_c_nc} unknown argument '${1}'."
        echo
        _errcode=1
    esac

    if [[ -n "${_errcode}" ]]; then
        __help
        exit ${_errcode}
    fi
done
