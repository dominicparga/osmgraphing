use crate::helpers::TestNode;
use osmgraphing::{network::NodeIdx, units::geo::Coordinate};

mod astar {
    use super::expected_paths;
    use crate::helpers::{assert_path, create_config, defaults, TestType};
    use osmgraphing::routing;

    #[test]
    fn bidirectional() {
        let cfg = create_config(TestType::SimpleStuttgart);

        let mut astar = routing::factory::astar::bidirectional::fastest(
            cfg.graph.edges.metrics.idx(&defaults::DURATION_ID.into()),
        );
        let expected_paths = expected_paths();

        assert_path(&mut astar, expected_paths, cfg.graph);
    }

    #[test]
    pub fn unidirectional() {
        let cfg = create_config(TestType::SimpleStuttgart);

        let mut astar = routing::factory::astar::unidirectional::fastest(
            cfg.graph.edges.metrics.idx(&defaults::DURATION_ID.into()),
        );
        let expected_paths = expected_paths();

        assert_path(&mut astar, expected_paths, cfg.graph);
    }
}

mod dijkstra {
    use super::expected_paths;
    use crate::helpers::{assert_path, create_config, defaults, TestType};
    use osmgraphing::routing;

    #[test]
    fn bidirectional() {
        let cfg = create_config(TestType::SimpleStuttgart);

        let mut dijkstra = routing::factory::dijkstra::bidirectional::fastest(
            cfg.graph.edges.metrics.idx(&defaults::DURATION_ID.into()),
        );
        let expected_paths = expected_paths();

        assert_path(&mut dijkstra, expected_paths, cfg.graph);
    }

    #[test]
    pub fn unidirectional() {
        let cfg = create_config(TestType::SimpleStuttgart);

        let mut dijkstra = routing::factory::dijkstra::unidirectional::fastest(
            cfg.graph.edges.metrics.idx(&defaults::DURATION_ID.into()),
        );
        let expected_paths = expected_paths();

        assert_path(&mut dijkstra, expected_paths, cfg.graph);
    }
}

fn expected_paths() -> Vec<(TestNode, TestNode, Option<(f32, Vec<Vec<TestNode>>)>)> {
    let opp: usize = 0;
    let bac: usize = 1;
    let wai: usize = 2;
    let end: usize = 3;
    let dea: usize = 4;
    let stu: usize = 5;

    let nodes: Vec<TestNode> = vec![
        ("opp", opp, 26_033_921, 48.9840100, 9.4589188),
        ("bac", bac, 26_160_028, 48.9416023, 9.4332023),
        ("wai", wai, 252_787_940, 48.8271096, 9.3098661),
        ("end", end, 298_249_467, 48.8108510, 9.3679493),
        ("dea", dea, 1_621_605_361, 48.9396327, 9.4188681),
        ("stu", stu, 2_933_335_353, 48.7701757, 9.1565768),
    ]
    .into_iter()
    .map(|(name, idx, id, lat, lon)| TestNode {
        name: String::from(name),
        idx: NodeIdx(idx),
        id,
        coord: Coordinate { lat, lon },
    })
    .collect();

    let expected_paths = vec![
        // opp
        (opp, opp, Some((0.0, vec![vec![]]))),
        (opp, bac, Some((576.0, vec![vec![opp, bac]]))),
        (opp, wai, Some((1_266.0, vec![vec![opp, bac, wai]]))),
        (opp, end, Some((1_566.0, vec![vec![opp, bac, end]]))),
        (opp, dea, Some((704.28, vec![vec![opp, bac, dea]]))),
        (opp, stu, Some((1_878.0, vec![vec![opp, bac, wai, stu]]))),
        // bac
        (bac, opp, Some((576.0, vec![vec![bac, opp]]))),
        (bac, bac, Some((0.0, vec![vec![]]))),
        (bac, wai, Some((690.0, vec![vec![bac, wai]]))),
        (bac, end, Some((990.0, vec![vec![bac, end]]))),
        (bac, dea, Some((128.28, vec![vec![bac, dea]]))),
        (bac, stu, Some((1_302.0, vec![vec![bac, wai, stu]]))),
        // wai
        (wai, opp, Some((1_266.0, vec![vec![wai, bac, opp]]))),
        (wai, bac, Some((690.0, vec![vec![wai, bac]]))),
        (wai, wai, Some((0.0, vec![vec![]]))),
        (wai, end, Some((576.0, vec![vec![wai, end]]))),
        (wai, dea, Some((818.28, vec![vec![wai, bac, dea]]))),
        (wai, stu, Some((612.0, vec![vec![wai, stu]]))),
        // end
        (end, opp, Some((1_566.0, vec![vec![end, bac, opp]]))),
        (end, bac, Some((990.0, vec![vec![end, bac]]))),
        (end, wai, Some((576.0, vec![vec![end, wai]]))),
        (end, end, Some((0.0, vec![vec![]]))),
        (end, dea, Some((1_118.28, vec![vec![end, bac, dea]]))),
        (end, stu, Some((945.0, vec![vec![end, stu]]))),
        // dea
        (dea, opp, None),
        (dea, bac, None),
        (dea, wai, None),
        (dea, end, None),
        (dea, dea, Some((0.0, vec![vec![]]))),
        (dea, stu, None),
        // stu
        (stu, opp, Some((1_878.0, vec![vec![stu, wai, bac, opp]]))),
        (stu, bac, Some((1_302.0, vec![vec![stu, wai, bac]]))),
        (stu, wai, Some((612.0, vec![vec![stu, wai]]))),
        (stu, end, Some((945.0, vec![vec![stu, end]]))),
        (stu, dea, Some((1_430.28, vec![vec![stu, wai, bac, dea]]))),
        (stu, stu, Some((0.0, vec![vec![]]))),
    ];

    // map indices to nodes
    expected_paths
        .into_iter()
        .map(|(src_idx, dst_idx, path_info)| {
            let src = nodes[src_idx].clone();
            let dst = nodes[dst_idx].clone();
            let path_info: Option<(f32, Vec<Vec<TestNode>>)> = match path_info {
                Some((cost, paths)) => {
                    let paths = paths
                        .into_iter()
                        .map(|path| {
                            path.into_iter()
                                .map(|node_idx| nodes[node_idx].clone())
                                .collect()
                        })
                        .collect();
                    Some((cost, paths))
                }
                None => None,
            };
            (src, dst, path_info)
        })
        .collect()
}