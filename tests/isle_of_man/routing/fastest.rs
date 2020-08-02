use crate::helpers::{compare_dijkstras, defaults, test_dijkstra, TestNode};
use defaults::paths::resources::isle_of_man as resources;
use osmgraphing::{
    configs::{self, routing::RoutingAlgo},
    defaults::capacity::DimVec,
    network::MetricIdx,
};

const METRIC_ID: &str = defaults::DURATION_ID;

#[test]
fn compare_dijkstras_on_ch_fmi_map() {
    compare_dijkstras(resources::CH_FMI_YAML, METRIC_ID);
}

#[test]
#[ignore]
fn chdijkstra_on_ch_fmi_map() {
    test_dijkstra(
        resources::CH_FMI_YAML,
        METRIC_ID,
        RoutingAlgo::CHDijkstra,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn dijkstra_on_ch_fmi_map() {
    test_dijkstra(
        resources::CH_FMI_YAML,
        METRIC_ID,
        RoutingAlgo::Dijkstra,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn chdijkstra_on_fmi_map() {
    test_dijkstra(
        resources::FMI_YAML,
        METRIC_ID,
        RoutingAlgo::CHDijkstra,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn dijkstra_on_fmi_map() {
    test_dijkstra(
        resources::FMI_YAML,
        METRIC_ID,
        RoutingAlgo::Dijkstra,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn chdijkstra_on_pbf_map() {
    test_dijkstra(
        resources::OSM_PBF_YAML,
        METRIC_ID,
        RoutingAlgo::CHDijkstra,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn dijkstra_on_pbf_map() {
    test_dijkstra(
        resources::OSM_PBF_YAML,
        METRIC_ID,
        RoutingAlgo::Dijkstra,
        Box::new(expected_paths),
    )
}

fn expected_paths(
    _cfg_parser: &configs::parsing::Config,
) -> Vec<(
    TestNode,
    TestNode,
    DimVec<MetricIdx>,
    Option<(DimVec<f64>, Vec<Vec<TestNode>>)>,
)> {
    unimplemented!("Testing routing on isle-of-man is not supported yet.")
}
