#!/usr/bin/env sh

#------------------------------------------------------------------------------#
# sources
# for sed-commands
# https://www.cyberciti.biz/faq/how-to-use-sed-to-find-and-replace-text-in-files-in-linux-unix-shell/

set -e

#------------------------------------------------------------------------------#
# helper-functions

ask_for_permission() {
    while true; do
        read -r yn
        case "${yn}" in
            'yes' | 'y' )
                return 0
            ;;
            'no' | 'n' )
                return 1
            ;;
            * )
                printf 'Please answer with [yes|y|no|n] '
            ;;
        esac
    done
}

replace_patterns_in_config() {
    cfg="${1}"

    # replace map_file
    sed -i 's|${map_file}|'"${map_file}"'|g' "${cfg}"

    # replace iter_0_dir
    sed -i 's|${iter_0_dir}|'"${iter_0_dir}"'|g' "${cfg}"

    # replace iter_1_dir
    sed -i 's|${iter_1_dir}|'"${iter_1_dir}"'|g' "${cfg}"

    # replace iter_dir
    sed -i 's|${iter_dir}|'"${iter_dir}"'|g' "${cfg}"

    # replace next_iter_dir
    sed -i 's|${next_iter_dir}|'"${next_iter_dir}"'|g' "${cfg}"
}

create_fmi_from_pbf() {
    cp "${cwd}/pbf_to_fmi.yaml" "${iter_0_dir}"
    cfg="${iter_0_dir}/pbf_to_fmi.yaml"

    cp "${cwd}/resources/configs/pbf_to_fmi.yaml" "${cfg}"
    replace_patterns_in_config "${cfg}"

    # parse and write graph
    "${osmgraphing_dir}/target/release/osmgraphing" \
    --config "${cfg}" \
    --writing-graph
}

create_route_pairs() {
    cp "${cwd}/writing_routes.yaml" "${iter_0_dir}"
    cfg="${iter_0_dir}/writing_routes.yaml"
    replace_patterns_in_config "${cfg}"

    # parse and write graph
    "${osmgraphing_dir}/target/release/osmgraphing" \
    --config "${cfg}" \
    --writing-routes
}

set_dim_in_multi_ch_constructor() {
    dim="${1}"

    sed -i \
    "s/^  static const size_t dim = .*;/  static const size_t dim = ${dim};/" \
    "${multi_ch_constructor_dir}/src/multi_lib/graph.hpp"
}

create_ch_fmi_from_iteration() {
    iter="${1}"
    if [ "${iter}" -eq '0' ]; then
        # with 3 metrics
        set_dim_in_multi_ch_constructor '3'
    else
        # with 4 metrics
        set_dim_in_multi_ch_constructor '4'
    fi

    cd "${multi_ch_constructor_dir}" || exit 1

    cmake -Bbuild
    cmake --build build

    ./build/multi-ch \
    --text "${iter_dir}/graph.fmi" \
    --percent "${percent}" \
    --stats \
    --write "${iter_dir}/graph.ch.fmi"

    cd "${osmgraphing_dir}" || exit 1
}

balance_for_iteration() {
    iter="${1}"
    if [ "${iter}" -eq '0' ]; then
        cp "${cwd}/balancer_0.yaml" "${iter_0_dir}"
        cfg="${iter_0_dir}/balancer_0.yaml"
    else
        cp "${cwd}/balancer_i.yaml" "${iter_dir}"
        cfg="${iter_dir}/balancer_i.yaml"
    fi
    replace_patterns_in_config "${cfg}"

    "${osmgraphing_dir}/target/release/balancer" --config "${cfg}"
}

#------------------------------------------------------------------------------#
# prepare and initialize

# set folders, relative to script
cwd="$(dirname "$(pwd)"/"${0}")"
osmgraphing_dir="${cwd}/../.."
custom_dir="${osmgraphing_dir}/custom"
multi_ch_constructor_dir="${osmgraphing_dir}/../multi-ch-constructor"
# multi-ch-constructor needs this value
percent='99.85' # 99.85

# get/set map-file
map_file="${osmgraphing_dir}/${1}"
if [ -z "${1}" ] || [ ! -f "${map_file}" ]; then
    printf "ERROR: Please provide a map-file relative to osmgraphing/. "
    echo "It's probably the one from your config-file."
    exit 1
fi
# get name of map by removing shortest suffix and longest prefix
map_name="${map_file%/*}"
map_name="${map_name##*/}"

# results-directory
results_dir="${custom_dir}/results/${map_name}/$(date "+%Y-%m-%d_%H-%M-%S")"
mkdir --verbose --parents "${results_dir}"

# build osmgraphing
"${osmgraphing_dir}/scripts/build.sh"

#------------------------------------------------------------------------------#
# iteration i

iter_0_dir="${results_dir}/0"
iter_1_dir="${results_dir}/1"
for iter in '0' '1' '2'; do
    iter_dir="${results_dir}/${iter}"

    if [ "${iter}" -eq '0' ]; then
        # create result-directory if first iteration
        mkdir --parents --verbose "${iter_0_dir}"

        # if map-file is osm-pbf-file
        # -> create fmi-map-file
        # else
        # -> link to existing fmi-map-file
        if ( echo "${map_file}" | grep ".osm.pbf" ); then
            create_fmi_from_pbf
        else
            ln --symbolic --verbose "${map_file}" "${iter_0_dir}"
        fi
    fi

    # create ch-fmi
    create_ch_fmi_from_iteration "${iter}"

    if [ "${iter}" -eq '0' ]; then
        create_route_pairs
    fi

    # create result-directory
    next_iter=$((iter + 1))
    next_iter_dir="${results_dir}/${next_iter}"
    mkdir --parents --verbose "${next_iter_dir}"

    # create results and new fmi-graph
    balance_for_iteration "${iter}"
done
