use crate::helpers::{compare_dijkstras, defaults, test_dijkstra, TestNode};
use kissunits::{distance::Kilometers, geo::Coordinate};
use osmgraphing::{
    configs::{self, SimpleId},
    defaults::capacity::DimVec,
    network::{MetricIdx, NodeIdx},
};
use smallvec::smallvec;

const METRIC_ID: &str = defaults::DISTANCE_ID;
const CONFIG: &str = defaults::paths::resources::configs::SMALL_FMI;
const CH_CONFIG: &str = defaults::paths::resources::configs::SMALL_CH_FMI;
const IS_CH_DIJKSTRA: bool = true;

#[test]
fn compare_dijkstras_on_ch_fmi_map() {
    compare_dijkstras(CH_CONFIG, METRIC_ID);
}

#[test]
fn chdijkstra_on_chmap() {
    test_dijkstra(
        CH_CONFIG,
        METRIC_ID,
        IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
fn dijkstra_on_chmap() {
    test_dijkstra(
        CH_CONFIG,
        METRIC_ID,
        !IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
fn chdijkstra_on_map() {
    test_dijkstra(CONFIG, METRIC_ID, IS_CH_DIJKSTRA, Box::new(expected_paths))
}

#[test]
fn dijkstra_on_map() {
    test_dijkstra(CONFIG, METRIC_ID, !IS_CH_DIJKSTRA, Box::new(expected_paths))
}

fn expected_paths(
    parsing_cfg: &configs::parsing::Config,
) -> Vec<(
    TestNode,
    TestNode,
    DimVec<MetricIdx>,
    Option<(DimVec<f64>, Vec<Vec<TestNode>>)>,
)> {
    let a: usize = 0;
    let b: usize = 1;
    let c: usize = 2;
    let d: usize = 3;
    let e: usize = 4;
    let f: usize = 5;
    let g: usize = 6;
    let h: usize = 7;

    let nodes: Vec<TestNode> = vec![
        ("a", a),
        ("b", b),
        ("c", c),
        ("d", d),
        ("e", e),
        ("f", f),
        ("g", g),
        ("h", h),
    ]
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
        // a
        (a, a, Some((0.0, vec![vec![]]))),
        (a, b, None),
        (a, c, None),
        (a, d, None),
        (a, e, None),
        (a, f, None),
        (a, g, None),
        (a, h, None),
        // b
        (b, a, Some((1.0, vec![vec![b, a]]))),
        (b, b, Some((0.0, vec![vec![]]))),
        (b, c, Some((1.0, vec![vec![b, c]]))),
        (b, d, None),
        (b, e, None),
        (b, f, None),
        (b, g, None),
        (b, h, None),
        // c
        (c, a, Some((1.0, vec![vec![c, a]]))),
        (c, b, Some((1.0, vec![vec![c, b]]))),
        (c, c, Some((0.0, vec![vec![]]))),
        (c, d, None),
        (c, e, None),
        (c, f, None),
        (c, g, None),
        (c, h, None),
        // d
        (d, a, Some((2.0, vec![vec![d, b, a]]))),
        (d, b, Some((1.0, vec![vec![d, b]]))),
        (d, c, Some((2.0, vec![vec![d, b, c]]))),
        (d, d, Some((0.0, vec![vec![]]))),
        (d, e, Some((2.0, vec![vec![d, e]]))),
        (d, f, Some((2.0, vec![vec![d, h, f]]))),
        (d, g, None),
        (d, h, Some((1.0, vec![vec![d, h]]))),
        // e
        (e, a, Some((4.0, vec![vec![e, d, b, a]]))),
        (e, b, Some((3.0, vec![vec![e, d, b]]))),
        (e, c, Some((4.0, vec![vec![e, d, b, c]]))),
        (e, d, Some((2.0, vec![vec![e, d]]))),
        (e, e, Some((0.0, vec![vec![]]))),
        (e, f, Some((1.0, vec![vec![e, f]]))),
        (e, g, None),
        (e, h, Some((2.0, vec![vec![e, f, h]]))),
        // f
        (f, a, Some((4.0, vec![vec![f, h, d, b, a]]))),
        (f, b, Some((3.0, vec![vec![f, h, d, b]]))),
        (f, c, Some((4.0, vec![vec![f, h, d, b, c]]))),
        (f, d, Some((2.0, vec![vec![f, h, d]]))),
        (f, e, Some((1.0, vec![vec![f, e]]))),
        (f, f, Some((0.0, vec![vec![]]))),
        (f, g, None),
        (f, h, Some((1.0, vec![vec![f, h]]))),
        // g
        (
            g,
            a,
            Some((5.0, vec![vec![g, e, d, b, a], vec![g, f, h, d, b, a]])),
        ),
        (
            g,
            b,
            Some((4.0, vec![vec![g, e, d, b], vec![g, f, h, d, b]])),
        ),
        (
            g,
            c,
            Some((5.0, vec![vec![g, e, d, b, c], vec![g, f, h, d, b, c]])),
        ),
        (g, d, Some((3.0, vec![vec![g, e, d], vec![g, f, h, d]]))),
        (g, e, Some((1.0, vec![vec![g, e]]))),
        (g, f, Some((1.0, vec![vec![g, f]]))),
        (g, g, Some((0.0, vec![vec![]]))),
        (g, h, Some((2.0, vec![vec![g, f, h]]))),
        // h
        (h, a, Some((3.0, vec![vec![h, d, b, a]]))),
        (h, b, Some((2.0, vec![vec![h, d, b]]))),
        (h, c, Some((3.0, vec![vec![h, d, b, c]]))),
        (h, d, Some((1.0, vec![vec![h, d]]))),
        (h, e, Some((2.0, vec![vec![h, f, e]]))),
        (h, f, Some((1.0, vec![vec![h, f]]))),
        (h, g, None),
        (h, h, Some((0.0, vec![vec![]]))),
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
                        .unwrap()
                )],
                path_info,
            )
        })
        .collect()
}
