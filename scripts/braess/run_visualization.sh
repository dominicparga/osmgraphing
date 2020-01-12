#!/usr/bin/env sh

clear

if [ -z "${1}" ]; then
    echo "Please provide a base-directory, e.g."
    echo "custom/out/braess/stuttgart-regbez_2019-09-07/2020-01-12/13-15-52/"
    exit 1
fi
__BASE_DIR="${1}"

#------------------------------------------------------------------------------#
# visualize route-counts

# route-counts
./scripts/braess/visualization.py \
'route-counts' \
"${__BASE_DIR}/loop_0/edge_stats.csv" \
'graph unchanged' \
"${__BASE_DIR}/loop_1/edge_stats.csv" \
'graph changed' \
"${__BASE_DIR}/route_counts_0_1.png"

#------------------------------------------------------------------------------#
# visualize src-dst

# src-dst 0
./scripts/braess/visualization.py \
'src-dst' \
"${__BASE_DIR}/loop_0/edge_stats.csv" \
'graph unchanged' \
"${__BASE_DIR}/loop_0/src_dst.png"

# src-dst 1
./scripts/braess/visualization.py \
'src-dst' \
"${__BASE_DIR}/loop_1/edge_stats.csv" \
'graph changed' \
"${__BASE_DIR}/loop_1/src_dst.png"

#------------------------------------------------------------------------------#
# utilization

# utilization 0
./scripts/braess/visualization.py \
'utilization' \
"${__BASE_DIR}/loop_0/edge_stats.csv" \
'graph unchanged' \
"${__BASE_DIR}/loop_0/utilization.png"

# utilization 1
./scripts/braess/visualization.py \
'utilization' \
"${__BASE_DIR}/loop_1/edge_stats.csv" \
'graph changed' \
"${__BASE_DIR}/loop_1/utilization.png"
