use crate::helpers::{defaults, TestNode, test_dijkstra, compare_dijkstras};
use osmgraphing::{configs, defaults::capacity::DimVec, network::MetricIdx};

const METRIC_ID: &str = defaults::DURATION_ID;
const PBF_CONFIG: &str = defaults::paths::resources::configs::ISLE_OF_MAN_PBF;
const CH_FMI_CONFIG: &str = defaults::paths::resources::configs::ISLE_OF_MAN_CH_FMI;
const FMI_CONFIG: &str = defaults::paths::resources::configs::ISLE_OF_MAN_FMI;
const IS_CH_DIJKSTRA: bool = true;

#[test]
fn compare_dijkstras_on_ch_fmi_map() {
    compare_dijkstras(CH_FMI_CONFIG, METRIC_ID);
}

#[test]
#[ignore]
fn chdijkstra_on_ch_fmi_map() {
    test_dijkstra(
        CH_FMI_CONFIG,
        METRIC_ID,
        IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn dijkstra_on_ch_fmi_map() {
    test_dijkstra(
        CH_FMI_CONFIG,
        METRIC_ID,
        !IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn chdijkstra_on_fmi_map() {
    test_dijkstra(
        FMI_CONFIG,
        METRIC_ID,
        IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn dijkstra_on_fmi_map() {
    test_dijkstra(
        FMI_CONFIG,
        METRIC_ID,
        !IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn chdijkstra_on_pbf_map() {
    test_dijkstra(
        PBF_CONFIG,
        METRIC_ID,
        IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn dijkstra_on_pbf_map() {
    test_dijkstra(
        PBF_CONFIG,
        METRIC_ID,
        !IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

fn expected_paths(
    _cfg_parser: &configs::parser::Config,
) -> Vec<(
    TestNode,
    TestNode,
    DimVec<MetricIdx>,
    Option<(DimVec<f64>, Vec<Vec<TestNode>>)>,
)> {
    unimplemented!("Testing routing on isle-of-man is not supported yet.")
}
