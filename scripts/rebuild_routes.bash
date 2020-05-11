#!/usr/bin/env bash

set -e

# build

cur_dir="$(dirname "$(pwd)"/"${0}")"
"${cur_dir}/build.sh"

root="${cur_dir}/.."
resources="${root}/resources"

for file in \
    "bidirectional-bait.fmi" \
    "isle-of-man_2020-03-14.pbf" \
    "isle-of-man_2020-03-14.fmi" \
    "isle-of-man_2020-03-14.ch.fmi" \
    "simple-stuttgart.fmi" \
    "small.fmi" \
    "small.ch.fmi"
do
    tmp="${resources}/routes/${file}.route-pairs"
    if [ -e "${tmp}" ]; then
        rm "${tmp}"
    fi

    tmp="${resources}/configs/${file}.yaml"
    cargo run --bin osmgraphing -- --config "${tmp}" --writing-routes --log WARN
done
