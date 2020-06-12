#!/usr/bin/env bash

set -e

# build

cwd="$(dirname "$(pwd)"/"${0}")"
cargo build --release
osmgraphing_dir="${cwd}/.."

for map_name in \
    "bidirectional_bait" \
    "isle_of_man_2020-03-14" \
    "simple_stuttgart" \
    "small"
do
    map_dir="${osmgraphing_dir}/resources/${map_name}"

    for map_type in \
        "fmi" \
        "ch.fmi" \
        "osm.pbf"
    do
        cfg="${map_dir}/${map_type}.yaml"

        if [ -e "${cfg}" ]; then
            # remove existing route-pairs-files

            # get name of it
            route_pairs_file="${osmgraphing_dir}/$(\
                grep "    file: .*route-pairs'" "${cfg}" |\
                sed -e "s|^    file: '\(.*\)'$|\1|" \
            )"
            if [ -e "${route_pairs_file}" ]; then
                rm "${route_pairs_file}"
            fi

            # create new ones
            cargo run --bin osmgraphing -- --config "${cfg}" --writing-routes
        fi
    done
done
