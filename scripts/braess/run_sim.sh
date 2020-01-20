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

cargo run --release -- \
braess \
--map "custom/resources/maps/${__MAP}" \
--proto-routes "${__OUT_DIR}/proto_routes_30k.csv" \
--out "${__OUT_DIR}" \
--threads 16 \
--loop-count 2 \
--removed-edges-per-loop 10 \
--route-count 1000
