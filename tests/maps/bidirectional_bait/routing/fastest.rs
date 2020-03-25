use crate::helpers::{defaults, test_dijkstra, TestNode};
use osmgraphing::{
    configs::{self, SimpleId},
    defaults::capacity::DimVec,
    network::{MetricIdx, NodeIdx},
    units::{
        geo::Coordinate,
        time::{Minutes, Seconds},
    },
};
use smallvec::smallvec;

const METRIC_ID: &str = defaults::DURATION_ID;
const CONFIG: &str = defaults::paths::resources::configs::BIDIRECTIONAL_BAIT_FMI;
const IS_CH_DIJKSTRA: bool = true;

#[test]
fn chdijkstra_on_map() {
    test_dijkstra(CONFIG, METRIC_ID, IS_CH_DIJKSTRA, Box::new(expected_paths))
}

#[test]
fn dijkstra_on_map() {
    test_dijkstra(CONFIG, METRIC_ID, !IS_CH_DIJKSTRA, Box::new(expected_paths))
}

fn expected_paths(
    cfg_parser: &configs::parser::Config,
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
            level: 0,
        })
        .collect();

    let expected_paths = vec![
        // ll
        (ll, ll, Some((0.0, vec![vec![]]))),
        (ll, bb, Some((0.6, vec![vec![ll, bb]]))),
        (ll, rr, Some((1.08, vec![vec![ll, tl, tr, rr]]))),
        (ll, tr, Some((0.72, vec![vec![ll, tl, tr]]))),
        (ll, tl, Some((0.36, vec![vec![ll, tl]]))),
        // bb
        (bb, ll, Some((0.6, vec![vec![bb, ll]]))),
        (bb, bb, Some((0.0, vec![vec![]]))),
        (bb, rr, Some((0.6, vec![vec![bb, rr]]))),
        (bb, tr, Some((0.96, vec![vec![bb, rr, tr]]))),
        (bb, tl, Some((0.96, vec![vec![bb, ll, tl]]))),
        // rr
        (rr, ll, Some((1.08, vec![vec![rr, tr, tl, ll]]))),
        (rr, bb, Some((0.6, vec![vec![rr, bb]]))),
        (rr, rr, Some((0.0, vec![vec![]]))),
        (rr, tr, Some((0.36, vec![vec![rr, tr]]))),
        (rr, tl, Some((0.72, vec![vec![rr, tr, tl]]))),
        // tr
        (tr, ll, Some((0.72, vec![vec![tr, tl, ll]]))),
        (tr, bb, Some((0.96, vec![vec![tr, rr, bb]]))),
        (tr, rr, Some((0.36, vec![vec![tr, rr]]))),
        (tr, tr, Some((0.0, vec![vec![]]))),
        (tr, tl, Some((0.36, vec![vec![tr, tl]]))),
        // tl
        (tl, ll, Some((0.36, vec![vec![tl, ll]]))),
        (tl, bb, Some((0.96, vec![vec![tl, ll, bb]]))),
        (tl, rr, Some((0.72, vec![vec![tl, tr, rr]]))),
        (tl, tr, Some((0.36, vec![vec![tl, tr]]))),
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
                    let cost = Minutes::from(Seconds(cost));
                    Some((smallvec![*cost], paths))
                }
                None => None,
            };
            (
                src,
                dst,
                smallvec![cfg_parser.edges.metric_idx(&SimpleId::from(METRIC_ID))],
                path_info,
            )
        })
        .collect()
}
