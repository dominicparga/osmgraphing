use crate::helpers::{defaults, test_dijkstra, TestNode};
use defaults::paths::resources::bidirectional_bait as resources;
use kissunits::{distance::Kilometers, geo::Coordinate};
use osmgraphing::{
    configs::{self, routing::RoutingAlgo, SimpleId},
    defaults::capacity::DimVec,
    network::{MetricIdx, NodeIdx},
};
use smallvec::smallvec;

const METRIC_ID: &str = defaults::DISTANCE_ID;

#[test]
fn chdijkstra_on_map() {
    test_dijkstra(
        resources::FMI_YAML,
        METRIC_ID,
        RoutingAlgo::CHDijkstra,
        Box::new(expected_paths),
    )
}

#[test]
fn dijkstra_on_map() {
    test_dijkstra(
        resources::FMI_YAML,
        METRIC_ID,
        RoutingAlgo::Dijkstra,
        Box::new(expected_paths),
    )
}

fn expected_paths(
    parsing_cfg: &configs::parsing::Config,
) -> Vec<(
    TestNode,
    TestNode,
    DimVec<MetricIdx>,
    Option<(DimVec<f64>, Vec<Vec<TestNode>>)>,
)> {
    // ll left
    // bb bottom
    // rr right
    // tr top-right
    // tl top-left
    let ll: usize = 0;
    let bb: usize = 1;
    let rr: usize = 2;
    let tr: usize = 3;
    let tl: usize = 4;

    let nodes: Vec<TestNode> = vec![("ll", ll), ("bb", bb), ("rr", rr), ("tr", tr), ("tl", tl)]
        .into_iter()
        .map(|(name, idx)| TestNode {
            name: String::from(name),
            idx: NodeIdx(idx),
            id: idx as i64,
            coord: Coordinate::zero(),
            ch_level: 0,
        })
        .collect();

    let expected_paths = vec![
        // ll
        (ll, ll, Some((0.0, vec![vec![]]))),
        (ll, bb, Some((5.0, vec![vec![ll, bb]]))),
        (ll, rr, Some((9.0, vec![vec![ll, tl, tr, rr]]))),
        (ll, tr, Some((6.0, vec![vec![ll, tl, tr]]))),
        (ll, tl, Some((3.0, vec![vec![ll, tl]]))),
        // bb
        (bb, ll, Some((5.0, vec![vec![bb, ll]]))),
        (bb, bb, Some((0.0, vec![vec![]]))),
        (bb, rr, Some((5.0, vec![vec![bb, rr]]))),
        (bb, tr, Some((8.0, vec![vec![bb, rr, tr]]))),
        (bb, tl, Some((8.0, vec![vec![bb, ll, tl]]))),
        // rr
        (rr, ll, Some((9.0, vec![vec![rr, tr, tl, ll]]))),
        (rr, bb, Some((5.0, vec![vec![rr, bb]]))),
        (rr, rr, Some((0.0, vec![vec![]]))),
        (rr, tr, Some((3.0, vec![vec![rr, tr]]))),
        (rr, tl, Some((6.0, vec![vec![rr, tr, tl]]))),
        // tr
        (tr, ll, Some((6.0, vec![vec![tr, tl, ll]]))),
        (tr, bb, Some((8.0, vec![vec![tr, rr, bb]]))),
        (tr, rr, Some((3.0, vec![vec![tr, rr]]))),
        (tr, tr, Some((0.0, vec![vec![]]))),
        (tr, tl, Some((3.0, vec![vec![tr, tl]]))),
        // tl
        (tl, ll, Some((3.0, vec![vec![tl, ll]]))),
        (tl, bb, Some((8.0, vec![vec![tl, ll, bb]]))),
        (tl, rr, Some((6.0, vec![vec![tl, tr, rr]]))),
        (tl, tr, Some((3.0, vec![vec![tl, tr]]))),
        (tl, tl, Some((0.0, vec![vec![]]))),
    ];

    // map indices to nodes
    expected_paths
        .into_iter()
        .map(|(src_idx, dst_idx, path_info)| {
            let src = nodes[src_idx].clone();
            let dst = nodes[dst_idx].clone();
            let path_info: Option<(DimVec<f64>, Vec<Vec<TestNode>>)> = match path_info {
                Some((cost, paths)) => {
                    let paths = paths
                        .into_iter()
                        .map(|path| {
                            path.into_iter()
                                .map(|node_idx| nodes[node_idx].clone())
                                .collect()
                        })
                        .collect();
                    let cost = Kilometers(cost / 1_000.0);
                    Some((smallvec![*cost], paths))
                }
                None => None,
            };
            (
                src,
                dst,
                smallvec![MetricIdx(
                    parsing_cfg
                        .edges
                        .metrics
                        .ids
                        .iter()
                        .position(|id| id == &SimpleId::from(METRIC_ID))
                        .expect("Expect bidrectional-bait's distance-id to be correct.")
                )],
                path_info,
            )
        })
        .collect()
}
