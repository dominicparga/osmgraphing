use super::{assert_correct, create_config, TestNode, TestType};
use osmgraphing::{network::NodeIdx, units::length::Meters};

mod astar {
    mod unidirectional {
        use super::super::{assert_correct, create_config, expected_paths, TestType};
        use osmgraphing::routing;

        #[test]
        fn simple_stuttgart() {
            let cfg = create_config(TestType::SimpleStuttgart);

            let mut astar = routing::factory::astar::unidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::SimpleStuttgart);

            assert_correct(&mut astar, expected_paths, cfg.graph);
        }

        #[test]
        fn small() {
            let cfg = create_config(TestType::Small);

            let mut astar = routing::factory::astar::unidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::Small);

            assert_correct(&mut astar, expected_paths, cfg.graph);
        }

        #[test]
        fn bait() {
            let cfg = create_config(TestType::BidirectionalBait);

            let mut astar = routing::factory::astar::unidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::BidirectionalBait);

            assert_correct(&mut astar, expected_paths, cfg.graph);
        }
    }

    mod bidirectional {
        use super::super::{assert_correct, create_config, expected_paths, TestType};
        use osmgraphing::routing;

        #[test]
        fn simple_stuttgart() {
            let cfg = create_config(TestType::SimpleStuttgart);

            let mut astar = routing::factory::astar::bidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::SimpleStuttgart);

            assert_correct(&mut astar, expected_paths, cfg.graph);
        }

        #[test]
        fn small() {
            let cfg = create_config(TestType::Small);

            let mut astar = routing::factory::astar::bidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::Small);

            assert_correct(&mut astar, expected_paths, cfg.graph);
        }

        #[test]
        fn bait() {
            let cfg = create_config(TestType::BidirectionalBait);

            let mut astar = routing::factory::astar::bidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::BidirectionalBait);

            assert_correct(&mut astar, expected_paths, cfg.graph);
        }
    }
}

//------------------------------------------------------------------------------------------------//

mod dijkstra {
    pub mod unidirectional {
        use super::super::{assert_correct, create_config, expected_paths, TestType};
        use osmgraphing::routing;

        #[test]
        fn simple_stuttgart() {
            let cfg = create_config(TestType::SimpleStuttgart);

            let mut dijkstra = routing::factory::dijkstra::unidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::SimpleStuttgart);

            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }

        #[test]
        fn small() {
            let cfg = create_config(TestType::Small);

            let mut dijkstra = routing::factory::dijkstra::unidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::Small);

            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }

        #[test]
        fn bait() {
            let cfg = create_config(TestType::BidirectionalBait);

            let mut dijkstra = routing::factory::dijkstra::unidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::BidirectionalBait);

            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }
    }

    pub mod bidirectional {
        use super::super::{assert_correct, create_config, expected_paths, TestType};
        use osmgraphing::routing;

        #[test]
        fn simple_stuttgart() {
            let cfg = create_config(TestType::SimpleStuttgart);

            let mut dijkstra = routing::factory::dijkstra::bidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::SimpleStuttgart);

            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }

        #[test]
        fn small() {
            let cfg = create_config(TestType::Small);

            let mut dijkstra = routing::factory::dijkstra::bidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::Small);

            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }

        #[test]
        fn bait() {
            let cfg = create_config(TestType::BidirectionalBait);

            let mut dijkstra = routing::factory::dijkstra::bidirectional::shortest(
                cfg.graph.edges.metrics.idx(&"length".into()),
            );
            let expected_paths = expected_paths(TestType::BidirectionalBait);

            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }
    }
}

//------------------------------------------------------------------------------------------------//

fn expected_paths(
    test_type: TestType,
) -> Vec<(TestNode, TestNode, Option<(Meters, Vec<Vec<TestNode>>)>)> {
    match test_type {
        TestType::BidirectionalBait => expected_paths_bait(),
        TestType::IsleOfMan => {
            unimplemented!("Testing routing on isle-of-man is not supported yet.")
        }
        TestType::SimpleStuttgart => expected_paths_simple_stuttgart(),
        TestType::Small => expected_paths_small(),
    }
}

fn expected_paths_simple_stuttgart(
) -> Vec<(TestNode, TestNode, Option<(Meters, Vec<Vec<TestNode>>)>)> {
    // (idx, id)
    let opp = TestNode {
        idx: NodeIdx::new(0),
        id: 26_033_921,
    };
    let bac = TestNode {
        idx: NodeIdx::new(1),
        id: 26_160_028,
    };
    let wai = TestNode {
        idx: NodeIdx::new(2),
        id: 252_787_940,
    };
    let end = TestNode {
        idx: NodeIdx::new(3),
        id: 298_249_467,
    };
    let dea = TestNode {
        idx: NodeIdx::new(4),
        id: 1_621_605_361,
    };
    let stu = TestNode {
        idx: NodeIdx::new(5),
        id: 2_933_335_353,
    };

    vec![
        // opp
        (opp, opp, Some((Meters::from(0u32), vec![vec![]]))),
        (
            opp,
            bac,
            Some((Meters::from(8_000u32), vec![vec![opp, bac]])),
        ),
        (
            opp,
            wai,
            Some((Meters::from(31_000u32), vec![vec![opp, bac, wai]])),
        ),
        (
            opp,
            end,
            Some((Meters::from(30_000u32), vec![vec![opp, bac, end]])),
        ),
        (
            opp,
            dea,
            Some((Meters::from(9_069u32), vec![vec![opp, bac, dea]])),
        ),
        (
            opp,
            stu,
            Some((Meters::from(48_000u32), vec![vec![opp, bac, wai, stu]])),
        ),
        // bac
        (
            bac,
            opp,
            Some((Meters::from(8_000u32), vec![vec![bac, opp]])),
        ),
        (bac, bac, Some((Meters::from(0u32), vec![vec![]]))),
        (
            bac,
            wai,
            Some((Meters::from(23_000u32), vec![vec![bac, wai]])),
        ),
        (
            bac,
            end,
            Some((Meters::from(22_000u32), vec![vec![bac, end]])),
        ),
        (
            bac,
            dea,
            Some((Meters::from(1_069u32), vec![vec![bac, dea]])),
        ),
        (
            bac,
            stu,
            Some((Meters::from(40_000u32), vec![vec![bac, wai, stu]])),
        ),
        // wai
        (
            wai,
            opp,
            Some((Meters::from(31_000u32), vec![vec![wai, bac, opp]])),
        ),
        (
            wai,
            bac,
            Some((Meters::from(23_000u32), vec![vec![wai, bac]])),
        ),
        (wai, wai, Some((Meters::from(0u32), vec![vec![]]))),
        (
            wai,
            end,
            Some((Meters::from(8_000u32), vec![vec![wai, end]])),
        ),
        (
            wai,
            dea,
            Some((Meters::from(24_069u32), vec![vec![wai, bac, dea]])),
        ),
        (
            wai,
            stu,
            Some((Meters::from(17_000u32), vec![vec![wai, stu]])),
        ),
        // end
        (
            end,
            opp,
            Some((Meters::from(30_000u32), vec![vec![end, bac, opp]])),
        ),
        (
            end,
            bac,
            Some((Meters::from(22_000u32), vec![vec![end, bac]])),
        ),
        (
            end,
            wai,
            Some((Meters::from(8_000u32), vec![vec![end, wai]])),
        ),
        (end, end, Some((Meters::from(0u32), vec![vec![]]))),
        (
            end,
            dea,
            Some((Meters::from(23_069u32), vec![vec![end, bac, dea]])),
        ),
        (
            end,
            stu,
            Some((Meters::from(21_000u32), vec![vec![end, stu]])),
        ),
        // dea
        (dea, opp, None),
        (dea, bac, None),
        (dea, wai, None),
        (dea, end, None),
        (dea, dea, Some((Meters::from(0u32), vec![vec![]]))),
        (dea, stu, None),
        // stu
        (
            stu,
            opp,
            Some((Meters::from(48_000u32), vec![vec![stu, wai, bac, opp]])),
        ),
        (
            stu,
            bac,
            Some((Meters::from(40_000u32), vec![vec![stu, wai, bac]])),
        ),
        (
            stu,
            wai,
            Some((Meters::from(17_000u32), vec![vec![stu, wai]])),
        ),
        (
            stu,
            end,
            Some((Meters::from(21_000u32), vec![vec![stu, end]])),
        ),
        (
            stu,
            dea,
            Some((Meters::from(41_069u32), vec![vec![stu, wai, bac, dea]])),
        ),
        (stu, stu, Some((Meters::from(0u32), vec![vec![]]))),
    ]
}

fn expected_paths_small() -> Vec<(TestNode, TestNode, Option<(Meters, Vec<Vec<TestNode>>)>)> {
    // (idx, id)
    let a = TestNode {
        idx: NodeIdx::new(0),
        id: 0,
    };
    let b = TestNode {
        idx: NodeIdx::new(1),
        id: 1,
    };
    let c = TestNode {
        idx: NodeIdx::new(2),
        id: 2,
    };
    let d = TestNode {
        idx: NodeIdx::new(3),
        id: 3,
    };
    let e = TestNode {
        idx: NodeIdx::new(4),
        id: 4,
    };
    let f = TestNode {
        idx: NodeIdx::new(5),
        id: 5,
    };
    let g = TestNode {
        idx: NodeIdx::new(6),
        id: 6,
    };
    let h = TestNode {
        idx: NodeIdx::new(7),
        id: 7,
    };

    vec![
        // a
        (a, a, Some((Meters::from(0u32), vec![vec![]]))),
        (a, b, None),
        (a, c, None),
        (a, d, None),
        (a, e, None),
        (a, f, None),
        (a, g, None),
        (a, h, None),
        // b
        (b, a, Some((Meters::from(1u32), vec![vec![b, a]]))),
        (b, b, Some((Meters::from(0u32), vec![vec![]]))),
        (b, c, Some((Meters::from(1u32), vec![vec![b, c]]))),
        (b, d, None),
        (b, e, None),
        (b, f, None),
        (b, g, None),
        (b, h, None),
        // c
        (c, a, Some((Meters::from(1u32), vec![vec![c, a]]))),
        (c, b, Some((Meters::from(1u32), vec![vec![c, b]]))),
        (c, c, Some((Meters::from(0u32), vec![vec![]]))),
        (c, d, None),
        (c, e, None),
        (c, f, None),
        (c, g, None),
        (c, h, None),
        // d
        (d, a, Some((Meters::from(2u32), vec![vec![d, b, a]]))),
        (d, b, Some((Meters::from(1u32), vec![vec![d, b]]))),
        (d, c, Some((Meters::from(2u32), vec![vec![d, b, c]]))),
        (d, d, Some((Meters::from(0u32), vec![vec![]]))),
        (d, e, Some((Meters::from(2u32), vec![vec![d, e]]))),
        (d, f, Some((Meters::from(2u32), vec![vec![d, h, f]]))),
        (d, g, None),
        (d, h, Some((Meters::from(1u32), vec![vec![d, h]]))),
        // e
        (e, a, Some((Meters::from(4u32), vec![vec![e, d, b, a]]))),
        (e, b, Some((Meters::from(3u32), vec![vec![e, d, b]]))),
        (e, c, Some((Meters::from(4u32), vec![vec![e, d, b, c]]))),
        (e, d, Some((Meters::from(2u32), vec![vec![e, d]]))),
        (e, e, Some((Meters::from(0u32), vec![vec![]]))),
        (e, f, Some((Meters::from(1u32), vec![vec![e, f]]))),
        (e, g, None),
        (e, h, Some((Meters::from(2u32), vec![vec![e, f, h]]))),
        // f
        (f, a, Some((Meters::from(4u32), vec![vec![f, h, d, b, a]]))),
        (f, b, Some((Meters::from(3u32), vec![vec![f, h, d, b]]))),
        (f, c, Some((Meters::from(4u32), vec![vec![f, h, d, b, c]]))),
        (f, d, Some((Meters::from(2u32), vec![vec![f, h, d]]))),
        (f, e, Some((Meters::from(1u32), vec![vec![f, e]]))),
        (f, f, Some((Meters::from(0u32), vec![vec![]]))),
        (f, g, None),
        (f, h, Some((Meters::from(1u32), vec![vec![f, h]]))),
        // g
        (g, a, Some((Meters::from(5u32), vec![vec![g, e, d, b, a]]))),
        (
            g,
            b,
            Some((
                Meters::from(4u32),
                vec![vec![g, e, d, b], vec![g, f, h, d, b]],
            )),
        ),
        (g, c, Some((Meters::from(5u32), vec![vec![g, e, d, b, c]]))),
        (
            g,
            d,
            Some((Meters::from(3u32), vec![vec![g, e, d], vec![g, f, d]])),
        ),
        (g, e, Some((Meters::from(1u32), vec![vec![g, e]]))),
        (g, f, Some((Meters::from(1u32), vec![vec![g, f]]))),
        (g, g, Some((Meters::from(0u32), vec![vec![]]))),
        (g, h, Some((Meters::from(2u32), vec![vec![g, f, h]]))),
        // h
        (h, a, Some((Meters::from(3u32), vec![vec![h, d, b, a]]))),
        (h, b, Some((Meters::from(2u32), vec![vec![h, d, b]]))),
        (h, c, Some((Meters::from(3u32), vec![vec![h, d, b, c]]))),
        (h, d, Some((Meters::from(1u32), vec![vec![h, d]]))),
        (h, e, Some((Meters::from(2u32), vec![vec![h, f, e]]))),
        (h, f, Some((Meters::from(1u32), vec![vec![h, f]]))),
        (h, g, None),
        (h, h, Some((Meters::from(0u32), vec![vec![]]))),
    ]
}

/// Consider a path from left to right.
/// It is important to have the smaller hop-distance at the bottom-path,
/// but the smaller weight-distance at the top-path.
fn expected_paths_bait() -> Vec<(TestNode, TestNode, Option<(Meters, Vec<Vec<TestNode>>)>)> {
    // (idx, id)
    // ll left
    // bb bottom
    // rr right
    // tr top-right
    // tl top-left
    let ll = TestNode {
        idx: NodeIdx::new(0),
        id: 0,
    };
    let bb = TestNode {
        idx: NodeIdx::new(1),
        id: 1,
    };
    let rr = TestNode {
        idx: NodeIdx::new(2),
        id: 2,
    };
    let tr = TestNode {
        idx: NodeIdx::new(3),
        id: 3,
    };
    let tl = TestNode {
        idx: NodeIdx::new(4),
        id: 4,
    };

    vec![
        // ll
        (ll, ll, Some((Meters::from(0u32), vec![vec![]]))),
        (ll, bb, Some((Meters::from(5u32), vec![vec![ll, bb]]))),
        (
            ll,
            rr,
            Some((Meters::from(9u32), vec![vec![ll, tl, tr, rr]])),
        ),
        (ll, tr, Some((Meters::from(6u32), vec![vec![ll, tl, tr]]))),
        (ll, tl, Some((Meters::from(3u32), vec![vec![ll, tl]]))),
        // bb
        (bb, ll, Some((Meters::from(5u32), vec![vec![bb, ll]]))),
        (bb, bb, Some((Meters::from(0u32), vec![vec![]]))),
        (bb, rr, Some((Meters::from(5u32), vec![vec![bb, rr]]))),
        (bb, tr, Some((Meters::from(8u32), vec![vec![bb, rr, tr]]))),
        (bb, tl, Some((Meters::from(8u32), vec![vec![bb, ll, tl]]))),
        // rr
        (
            rr,
            ll,
            Some((Meters::from(9u32), vec![vec![rr, tr, tl, ll]])),
        ),
        (rr, bb, Some((Meters::from(5u32), vec![vec![rr, bb]]))),
        (rr, rr, Some((Meters::from(0u32), vec![vec![]]))),
        (rr, tr, Some((Meters::from(3u32), vec![vec![rr, tr]]))),
        (rr, tl, Some((Meters::from(6u32), vec![vec![rr, tr, tl]]))),
        // tr
        (tr, ll, Some((Meters::from(6u32), vec![vec![tr, tl, ll]]))),
        (tr, bb, Some((Meters::from(8u32), vec![vec![tr, rr, bb]]))),
        (tr, rr, Some((Meters::from(3u32), vec![vec![tr, rr]]))),
        (tr, tr, Some((Meters::from(0u32), vec![vec![]]))),
        (tr, tl, Some((Meters::from(3u32), vec![vec![tr, tl]]))),
        // tl
        (tl, ll, Some((Meters::from(3u32), vec![vec![tl, ll]]))),
        (tl, bb, Some((Meters::from(8u32), vec![vec![tl, ll, bb]]))),
        (tl, rr, Some((Meters::from(6u32), vec![vec![tl, tr, rr]]))),
        (tl, tr, Some((Meters::from(3u32), vec![vec![tl, tr]]))),
        (tl, tl, Some((Meters::from(0u32), vec![vec![]]))),
    ]
}
