use super::{assert_correct, create_config, TestNode, TestType};
use osmgraphing::{network::NodeIdx, units::time::Milliseconds};

//------------------------------------------------------------------------------------------------//

mod astar {
    mod unidirectional {
        use super::super::{assert_correct, create_config, expected_paths, TestType};
        use osmgraphing::routing;

        #[test]
        fn simple_stuttgart() {
            let mut astar = routing::factory::astar::unidirectional::fastest();
            let expected_paths = expected_paths(TestType::SimpleStuttgart);

            let cfg = create_config(TestType::SimpleStuttgart);
            assert_correct(&mut astar, expected_paths, cfg.graph);
        }

        #[test]
        fn small() {
            let mut astar = routing::factory::astar::unidirectional::fastest();
            let expected_paths = expected_paths(TestType::Small);

            let cfg = create_config(TestType::Small);
            assert_correct(&mut astar, expected_paths, cfg.graph);
        }

        #[test]
        fn bait() {
            let mut astar = routing::factory::astar::unidirectional::fastest();
            let expected_paths = expected_paths(TestType::BidirectionalBait);

            let cfg = create_config(TestType::BidirectionalBait);
            assert_correct(&mut astar, expected_paths, cfg.graph);
        }
    }

    mod bidirectional {
        use super::super::{assert_correct, create_config, expected_paths, TestType};
        use osmgraphing::routing;

        #[test]
        fn simple_stuttgart() {
            let mut astar = routing::factory::astar::bidirectional::fastest();
            let expected_paths = expected_paths(TestType::SimpleStuttgart);

            let cfg = create_config(TestType::SimpleStuttgart);
            assert_correct(&mut astar, expected_paths, cfg.graph);
        }

        #[test]
        fn small() {
            let mut astar = routing::factory::astar::bidirectional::fastest();
            let expected_paths = expected_paths(TestType::Small);

            let cfg = create_config(TestType::Small);
            assert_correct(&mut astar, expected_paths, cfg.graph);
        }

        #[test]
        fn bait() {
            let mut astar = routing::factory::astar::bidirectional::fastest();
            let expected_paths = expected_paths(TestType::BidirectionalBait);

            let cfg = create_config(TestType::BidirectionalBait);
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
            let mut dijkstra = routing::factory::dijkstra::unidirectional::fastest();
            let expected_paths = expected_paths(TestType::SimpleStuttgart);

            let cfg = create_config(TestType::SimpleStuttgart);
            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }

        #[test]
        fn small() {
            let mut dijkstra = routing::factory::dijkstra::unidirectional::fastest();
            let expected_paths = expected_paths(TestType::Small);

            let cfg = create_config(TestType::Small);
            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }

        #[test]
        fn bait() {
            let mut dijkstra = routing::factory::dijkstra::unidirectional::fastest();
            let expected_paths = expected_paths(TestType::BidirectionalBait);

            let cfg = create_config(TestType::BidirectionalBait);
            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }
    }

    pub mod bidirectional {
        use super::super::{assert_correct, create_config, expected_paths, TestType};
        use osmgraphing::routing;

        #[test]
        fn simple_stuttgart() {
            let mut dijkstra = routing::factory::dijkstra::bidirectional::fastest();
            let expected_paths = expected_paths(TestType::SimpleStuttgart);

            let cfg = create_config(TestType::SimpleStuttgart);
            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }

        #[test]
        fn small() {
            let mut dijkstra = routing::factory::dijkstra::bidirectional::fastest();
            let expected_paths = expected_paths(TestType::Small);

            let cfg = create_config(TestType::Small);
            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }

        #[test]
        fn bait() {
            let mut dijkstra = routing::factory::dijkstra::bidirectional::fastest();
            let expected_paths = expected_paths(TestType::BidirectionalBait);

            let cfg = create_config(TestType::BidirectionalBait);
            assert_correct(&mut dijkstra, expected_paths, cfg.graph);
        }
    }
}

//------------------------------------------------------------------------------------------------//

fn expected_paths(
    test_type: TestType,
) -> Vec<(
    TestNode,
    TestNode,
    Option<(Milliseconds, Vec<Vec<TestNode>>)>,
)> {
    match test_type {
        TestType::BidirectionalBait => expected_paths_bait(),
        TestType::IsleOfMan => {
            unimplemented!("Testing routing on isle-of-man is not supported yet.")
        }
        TestType::SimpleStuttgart => expected_paths_simple_stuttgart(),
        TestType::Small => expected_paths_small(),
    }
}

fn expected_paths_simple_stuttgart() -> Vec<(
    TestNode,
    TestNode,
    Option<(Milliseconds, Vec<Vec<TestNode>>)>,
)> {
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
        (opp, opp, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (
            opp,
            bac,
            Some((Milliseconds::from(576_000u32), vec![vec![opp, bac]])),
        ),
        (
            opp,
            wai,
            Some((Milliseconds::from(1_266_000u32), vec![vec![opp, bac, wai]])),
        ),
        (
            opp,
            end,
            Some((Milliseconds::from(1_566_000u32), vec![vec![opp, bac, end]])),
        ),
        (
            opp,
            dea,
            Some((Milliseconds::from(704_280u32), vec![vec![opp, bac, dea]])),
        ),
        (
            opp,
            stu,
            Some((
                Milliseconds::from(1_878_000u32),
                vec![vec![opp, bac, wai, stu]],
            )),
        ),
        // bac
        (
            bac,
            opp,
            Some((Milliseconds::from(576_000u32), vec![vec![bac, opp]])),
        ),
        (bac, bac, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (
            bac,
            wai,
            Some((Milliseconds::from(690_000u32), vec![vec![bac, wai]])),
        ),
        (
            bac,
            end,
            Some((Milliseconds::from(990_000u32), vec![vec![bac, end]])),
        ),
        (
            bac,
            dea,
            Some((Milliseconds::from(128_280u32), vec![vec![bac, dea]])),
        ),
        (
            bac,
            stu,
            Some((Milliseconds::from(1_302_000u32), vec![vec![bac, wai, stu]])),
        ),
        // wai
        (
            wai,
            opp,
            Some((Milliseconds::from(1_266_000u32), vec![vec![wai, bac, opp]])),
        ),
        (
            wai,
            bac,
            Some((Milliseconds::from(690_000u32), vec![vec![wai, bac]])),
        ),
        (wai, wai, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (
            wai,
            end,
            Some((Milliseconds::from(576_000u32), vec![vec![wai, end]])),
        ),
        (
            wai,
            dea,
            Some((Milliseconds::from(818_280u32), vec![vec![wai, bac, dea]])),
        ),
        (
            wai,
            stu,
            Some((Milliseconds::from(612_000u32), vec![vec![wai, stu]])),
        ),
        // end
        (
            end,
            opp,
            Some((Milliseconds::from(1_566_000u32), vec![vec![end, bac, opp]])),
        ),
        (
            end,
            bac,
            Some((Milliseconds::from(990_000u32), vec![vec![end, bac]])),
        ),
        (
            end,
            wai,
            Some((Milliseconds::from(576_000u32), vec![vec![end, wai]])),
        ),
        (end, end, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (
            end,
            dea,
            Some((Milliseconds::from(1_118_280u32), vec![vec![end, bac, dea]])),
        ),
        (
            end,
            stu,
            Some((Milliseconds::from(945_000u32), vec![vec![end, stu]])),
        ),
        // dea
        (dea, opp, None),
        (dea, bac, None),
        (dea, wai, None),
        (dea, end, None),
        (dea, dea, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (dea, stu, None),
        // stu
        (
            stu,
            opp,
            Some((
                Milliseconds::from(1_878_000u32),
                vec![vec![stu, wai, bac, opp]],
            )),
        ),
        (
            stu,
            bac,
            Some((Milliseconds::from(1_302_000u32), vec![vec![stu, wai, bac]])),
        ),
        (
            stu,
            wai,
            Some((Milliseconds::from(612_000u32), vec![vec![stu, wai]])),
        ),
        (
            stu,
            end,
            Some((Milliseconds::from(945_000u32), vec![vec![stu, end]])),
        ),
        (
            stu,
            dea,
            Some((
                Milliseconds::from(1_430_280u32),
                vec![vec![stu, wai, bac, dea]],
            )),
        ),
        (stu, stu, Some((Milliseconds::from(0u32), vec![vec![]]))),
    ]
}

fn expected_paths_small() -> Vec<(
    TestNode,
    TestNode,
    Option<(Milliseconds, Vec<Vec<TestNode>>)>,
)> {
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
        (a, a, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (a, b, None),
        (a, c, None),
        (a, d, None),
        (a, e, None),
        (a, f, None),
        (a, g, None),
        (a, h, None),
        // b
        (b, a, Some((Milliseconds::from(120u32), vec![vec![b, a]]))),
        (b, b, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (b, c, Some((Milliseconds::from(120u32), vec![vec![b, c]]))),
        (b, d, None),
        (b, e, None),
        (b, f, None),
        (b, g, None),
        (b, h, None),
        // c
        (c, a, Some((Milliseconds::from(120u32), vec![vec![c, a]]))),
        (c, b, Some((Milliseconds::from(120u32), vec![vec![c, b]]))),
        (c, c, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (c, d, None),
        (c, e, None),
        (c, f, None),
        (c, g, None),
        (c, h, None),
        // d
        (
            d,
            a,
            Some((Milliseconds::from(240u32), vec![vec![d, b, a]])),
        ),
        (d, b, Some((Milliseconds::from(120u32), vec![vec![d, b]]))),
        (
            d,
            c,
            Some((Milliseconds::from(240u32), vec![vec![d, b, c]])),
        ),
        (d, d, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (d, e, Some((Milliseconds::from(240u32), vec![vec![d, e]]))),
        (
            d,
            f,
            Some((Milliseconds::from(240u32), vec![vec![d, h, f]])),
        ),
        (d, g, None),
        (d, h, Some((Milliseconds::from(120u32), vec![vec![d, h]]))),
        // e
        (
            e,
            a,
            Some((Milliseconds::from(480u32), vec![vec![e, d, b, a]])),
        ),
        (
            e,
            b,
            Some((Milliseconds::from(360u32), vec![vec![e, d, b]])),
        ),
        (
            e,
            c,
            Some((Milliseconds::from(480u32), vec![vec![e, d, b, c]])),
        ),
        (e, d, Some((Milliseconds::from(240u32), vec![vec![e, d]]))),
        (e, e, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (e, f, Some((Milliseconds::from(120u32), vec![vec![e, f]]))),
        (e, g, None),
        (
            e,
            h,
            Some((Milliseconds::from(240u32), vec![vec![e, f, h]])),
        ),
        // f
        (
            f,
            a,
            Some((Milliseconds::from(480u32), vec![vec![f, h, d, b, a]])),
        ),
        (
            f,
            b,
            Some((Milliseconds::from(360u32), vec![vec![f, h, d, b]])),
        ),
        (
            f,
            c,
            Some((Milliseconds::from(480u32), vec![vec![f, h, d, b, c]])),
        ),
        (
            f,
            d,
            Some((Milliseconds::from(240u32), vec![vec![f, h, d]])),
        ),
        (f, e, Some((Milliseconds::from(120u32), vec![vec![f, e]]))),
        (f, f, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (f, g, None),
        (f, h, Some((Milliseconds::from(120u32), vec![vec![f, h]]))),
        // g
        (
            g,
            a,
            Some((
                Milliseconds::from(600u32),
                vec![vec![g, f, h, d, b, a], vec![g, e, d, b, a]],
            )),
        ),
        (
            g,
            b,
            Some((
                Milliseconds::from(480u32),
                vec![vec![g, e, d, b], vec![g, f, h, d, b]],
            )),
        ),
        (
            g,
            c,
            Some((
                Milliseconds::from(600u32),
                vec![vec![g, e, d, b, c], vec![g, f, h, d, b, c]],
            )),
        ),
        (
            g,
            d,
            Some((
                Milliseconds::from(360u32),
                vec![vec![g, e, d], vec![g, f, h, d]],
            )),
        ),
        (g, e, Some((Milliseconds::from(120u32), vec![vec![g, e]]))),
        (g, f, Some((Milliseconds::from(120u32), vec![vec![g, f]]))),
        (g, g, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (
            g,
            h,
            Some((Milliseconds::from(240u32), vec![vec![g, f, h]])),
        ),
        // h
        (
            h,
            a,
            Some((Milliseconds::from(360u32), vec![vec![h, d, b, a]])),
        ),
        (
            h,
            b,
            Some((Milliseconds::from(240u32), vec![vec![h, d, b]])),
        ),
        (
            h,
            c,
            Some((Milliseconds::from(360u32), vec![vec![h, d, b, c]])),
        ),
        (h, d, Some((Milliseconds::from(120u32), vec![vec![h, d]]))),
        (
            h,
            e,
            Some((Milliseconds::from(240u32), vec![vec![h, f, e]])),
        ),
        (h, f, Some((Milliseconds::from(120u32), vec![vec![h, f]]))),
        (h, g, None),
        (h, h, Some((Milliseconds::from(0u32), vec![vec![]]))),
    ]
}

/// Consider a path from left to right.
/// It is important to have the smaller hop-distance at the bottom-path,
/// but the smaller weight-distance at the top-path.
fn expected_paths_bait() -> Vec<(
    TestNode,
    TestNode,
    Option<(Milliseconds, Vec<Vec<TestNode>>)>,
)> {
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
        (ll, ll, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (
            ll,
            bb,
            Some((Milliseconds::from(600u32), vec![vec![ll, bb]])),
        ),
        (
            ll,
            rr,
            Some((Milliseconds::from(1080u32), vec![vec![ll, tl, tr, rr]])),
        ),
        (
            ll,
            tr,
            Some((Milliseconds::from(720u32), vec![vec![ll, tl, tr]])),
        ),
        (
            ll,
            tl,
            Some((Milliseconds::from(360u32), vec![vec![ll, tl]])),
        ),
        // bb
        (
            bb,
            ll,
            Some((Milliseconds::from(600u32), vec![vec![bb, ll]])),
        ),
        (bb, bb, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (
            bb,
            rr,
            Some((Milliseconds::from(600u32), vec![vec![bb, rr]])),
        ),
        (
            bb,
            tr,
            Some((Milliseconds::from(960u32), vec![vec![bb, rr, tr]])),
        ),
        (
            bb,
            tl,
            Some((Milliseconds::from(960u32), vec![vec![bb, ll, tl]])),
        ),
        // rr
        (
            rr,
            ll,
            Some((Milliseconds::from(1080u32), vec![vec![rr, tr, tl, ll]])),
        ),
        (
            rr,
            bb,
            Some((Milliseconds::from(600u32), vec![vec![rr, bb]])),
        ),
        (rr, rr, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (
            rr,
            tr,
            Some((Milliseconds::from(360u32), vec![vec![rr, tr]])),
        ),
        (
            rr,
            tl,
            Some((Milliseconds::from(720u32), vec![vec![rr, tr, tl]])),
        ),
        // tr
        (
            tr,
            ll,
            Some((Milliseconds::from(720u32), vec![vec![tr, tl, ll]])),
        ),
        (
            tr,
            bb,
            Some((Milliseconds::from(960u32), vec![vec![tr, rr, bb]])),
        ),
        (
            tr,
            rr,
            Some((Milliseconds::from(360u32), vec![vec![tr, rr]])),
        ),
        (tr, tr, Some((Milliseconds::from(0u32), vec![vec![]]))),
        (
            tr,
            tl,
            Some((Milliseconds::from(360u32), vec![vec![tr, tl]])),
        ),
        // tl
        (
            tl,
            ll,
            Some((Milliseconds::from(360u32), vec![vec![tl, ll]])),
        ),
        (
            tl,
            bb,
            Some((Milliseconds::from(960u32), vec![vec![tl, ll, bb]])),
        ),
        (
            tl,
            rr,
            Some((Milliseconds::from(720u32), vec![vec![tl, tr, rr]])),
        ),
        (
            tl,
            tr,
            Some((Milliseconds::from(360u32), vec![vec![tl, tr]])),
        ),
        (tl, tl, Some((Milliseconds::from(0u32), vec![vec![]]))),
    ]
}
