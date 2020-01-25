#!/usr/bin/env sh

#------------------------------------------------------------------------------#
# prepare directories for simulation-inputs and -outputs

__MAP='stuttgart-regbez_2019-09-07.osm.pbf'
# truncate extensions
__OUT_DIR="custom/out/braess/${__MAP%%.*}"

#------------------------------------------------------------------------------#
# clear console, cleanup project and build+run simulation

clear

cargo fmt

mkdir -v -p "${__OUT_DIR}"

cargo run --release -- \
gen-proto-routes \
--seed 42 \
--route-count 30000 \
--map "custom/resources/maps/${__MAP}" \
--out "${__OUT_DIR}/proto_routes_30k.csv" \
