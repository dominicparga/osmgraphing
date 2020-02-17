use super::{assert_correct, TestNode};
use osmgraphing::{network::NodeIdx, units::time::Milliseconds};

//------------------------------------------------------------------------------------------------//

mod astar {
    mod unidirectional {
        use super::super::{
            assert_correct, expected_paths_bait, expected_paths_simple_stuttgart,
            expected_paths_small,
        };
        use osmgraphing::{configs::graph, routing};

        #[test]
        fn simple_stuttgart() {
            let mut astar = routing::factory::astar::unidirectional::fastest();
            let expected_paths = expected_paths_simple_stuttgart();

            let mut cfg = graph::Config::default();
            cfg.paths_mut()
                .set_map_file("resources/maps/simple_stuttgart.fmi");

            assert_correct(&mut astar, expected_paths, &cfg);
        }

        #[test]
        fn small() {
            let mut astar = routing::factory::astar::unidirectional::fastest();
            let expected_paths = expected_paths_small();

            let mut cfg = graph::Config::default();
            cfg.paths_mut().set_map_file("resources/maps/small.fmi");

            assert_correct(&mut astar, expected_paths, &cfg);
        }

        #[test]
        fn bait() {
            let mut astar = routing::factory::astar::unidirectional::fastest();
            let expected_paths = expected_paths_bait();

            let mut cfg = graph::Config::default();
            cfg.paths_mut()
                .set_map_file("resources/maps/bidirectional_bait.fmi");

            assert_correct(&mut astar, expected_paths, &cfg);
        }
    }

    mod bidirectional {
        use super::super::{
            assert_correct, expected_paths_bait, expected_paths_simple_stuttgart,
            expected_paths_small,
        };
        use osmgraphing::{configs::graph, routing};

        #[test]
        fn simple_stuttgart() {
            let mut astar = routing::factory::astar::bidirectional::fastest();
            let expected_paths = expected_paths_simple_stuttgart();

            let mut cfg = graph::Config::default();
            cfg.paths_mut()
                .set_map_file("resources/maps/simple_stuttgart.fmi");

            assert_correct(&mut astar, expected_paths, &cfg);
        }

        #[test]
        fn small() {
            let mut astar = routing::factory::astar::bidirectional::fastest();
            let expected_paths = expected_paths_small();

            let mut cfg = graph::Config::default();
            cfg.paths_mut().set_map_file("resources/maps/small.fmi");

            assert_correct(&mut astar, expected_paths, &cfg);
        }

        #[test]
        fn bait() {
            let mut astar = routing::factory::astar::bidirectional::fastest();
            let expected_paths = expected_paths_bait();

            let mut cfg = graph::Config::default();
            cfg.paths_mut()
                .set_map_file("resources/maps/bidirectional_bait.fmi");

            assert_correct(&mut astar, expected_paths, &cfg);
        }
    }
}

//------------------------------------------------------------------------------------------------//

mod dijkstra {
    pub mod unidirectional {
        use super::super::{
            assert_correct, expected_paths_bait, expected_paths_simple_stuttgart,
            expected_paths_small,
        };
        use osmgraphing::{configs::graph, routing};

        #[test]
        fn simple_stuttgart() {
            let mut dijkstra = routing::factory::dijkstra::unidirectional::fastest();
            let expected_paths = expected_paths_simple_stuttgart();

            let mut cfg = graph::Config::default();
            cfg.paths_mut()
                .set_map_file("resources/maps/simple_stuttgart.fmi");

            assert_correct(&mut dijkstra, expected_paths, &cfg);
        }

        #[test]
        fn small() {
            let mut dijkstra = routing::factory::dijkstra::unidirectional::fastest();
            let expected_paths = expected_paths_small();

            let mut cfg = graph::Config::default();
            cfg.paths_mut().set_map_file("resources/maps/small.fmi");

            assert_correct(&mut dijkstra, expected_paths, &cfg);
        }

        #[test]
        fn bait() {
            let mut dijkstra = routing::factory::dijkstra::unidirectional::fastest();
            let expected_paths = expected_paths_bait();

            let mut cfg = graph::Config::default();
            cfg.paths_mut()
                .set_map_file("resources/maps/bidirectional_bait.fmi");

            assert_correct(&mut dijkstra, expected_paths, &cfg);
        }
    }

    pub mod bidirectional {
        use super::super::{
            assert_correct, expected_paths_bait, expected_paths_simple_stuttgart,
            expected_paths_small,
        };
        use osmgraphing::{configs::graph, routing};

        #[test]
        fn simple_stuttgart() {
            let mut dijkstra = routing::factory::dijkstra::bidirectional::fastest();
            let expected_paths = expected_paths_simple_stuttgart();

            let mut cfg = graph::Config::default();
            cfg.paths_mut()
                .set_map_file("resources/maps/simple_stuttgart.fmi");

            assert_correct(&mut dijkstra, expected_paths, &cfg);
        }

        #[test]
        fn small() {
            let mut dijkstra = routing::factory::dijkstra::bidirectional::fastest();
            let expected_paths = expected_paths_small();

            let mut cfg = graph::Config::default();
            cfg.paths_mut().set_map_file("resources/maps/small.fmi");

            assert_correct(&mut dijkstra, expected_paths, &cfg);
        }

        #[test]
        fn bait() {
            let mut dijkstra = routing::factory::dijkstra::bidirectional::fastest();
            let expected_paths = expected_paths_bait();

            let mut cfg = graph::Config::default();
            cfg.paths_mut()
                .set_map_file("resources/maps/bidirectional_bait.fmi");

            assert_correct(&mut dijkstra, expected_paths, &cfg);
        }
    }
}

//------------------------------------------------------------------------------------------------//

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
        (opp, opp, Some((Milliseconds::new(0), vec![vec![]]))),
        (
            opp,
            bac,
            Some((Milliseconds::new(576_000), vec![vec![opp, bac]])),
        ),
        (
            opp,
            wai,
            Some((Milliseconds::new(1_266_000), vec![vec![opp, bac, wai]])),
        ),
        (
            opp,
            end,
            Some((Milliseconds::new(1_566_000), vec![vec![opp, bac, end]])),
        ),
        (
            opp,
            dea,
            Some((Milliseconds::new(704_280), vec![vec![opp, bac, dea]])),
        ),
        (
            opp,
            stu,
            Some((Milliseconds::new(1_878_000), vec![vec![opp, bac, wai, stu]])),
        ),
        // bac
        (
            bac,
            opp,
            Some((Milliseconds::new(576_000), vec![vec![bac, opp]])),
        ),
        (bac, bac, Some((Milliseconds::new(0), vec![vec![]]))),
        (
            bac,
            wai,
            Some((Milliseconds::new(690_000), vec![vec![bac, wai]])),
        ),
        (
            bac,
            end,
            Some((Milliseconds::new(990_000), vec![vec![bac, end]])),
        ),
        (
            bac,
            dea,
            Some((Milliseconds::new(128_280), vec![vec![bac, dea]])),
        ),
        (
            bac,
            stu,
            Some((Milliseconds::new(1_302_000), vec![vec![bac, wai, stu]])),
        ),
        // wai
        (
            wai,
            opp,
            Some((Milliseconds::new(1_266_000), vec![vec![wai, bac, opp]])),
        ),
        (
            wai,
            bac,
            Some((Milliseconds::new(690_000), vec![vec![wai, bac]])),
        ),
        (wai, wai, Some((Milliseconds::new(0), vec![vec![]]))),
        (
            wai,
            end,
            Some((Milliseconds::new(576_000), vec![vec![wai, end]])),
        ),
        (
            wai,
            dea,
            Some((Milliseconds::new(818_280), vec![vec![wai, bac, dea]])),
        ),
        (
            wai,
            stu,
            Some((Milliseconds::new(612_000), vec![vec![wai, stu]])),
        ),
        // end
        (
            end,
            opp,
            Some((Milliseconds::new(1_566_000), vec![vec![end, bac, opp]])),
        ),
        (
            end,
            bac,
            Some((Milliseconds::new(990_000), vec![vec![end, bac]])),
        ),
        (
            end,
            wai,
            Some((Milliseconds::new(576_000), vec![vec![end, wai]])),
        ),
        (end, end, Some((Milliseconds::new(0), vec![vec![]]))),
        (
            end,
            dea,
            Some((Milliseconds::new(1_118_280), vec![vec![end, bac, dea]])),
        ),
        (
            end,
            stu,
            Some((Milliseconds::new(945_000), vec![vec![end, stu]])),
        ),
        // dea
        (dea, opp, None),
        (dea, bac, None),
        (dea, wai, None),
        (dea, end, None),
        (dea, dea, Some((Milliseconds::new(0), vec![vec![]]))),
        (dea, stu, None),
        // stu
        (
            stu,
            opp,
            Some((Milliseconds::new(1_878_000), vec![vec![stu, wai, bac, opp]])),
        ),
        (
            stu,
            bac,
            Some((Milliseconds::new(1_302_000), vec![vec![stu, wai, bac]])),
        ),
        (
            stu,
            wai,
            Some((Milliseconds::new(612_000), vec![vec![stu, wai]])),
        ),
        (
            stu,
            end,
            Some((Milliseconds::new(945_000), vec![vec![stu, end]])),
        ),
        (
            stu,
            dea,
            Some((Milliseconds::new(1_430_280), vec![vec![stu, wai, bac, dea]])),
        ),
        (stu, stu, Some((Milliseconds::new(0), vec![vec![]]))),
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
        (a, a, Some((Milliseconds::new(0), vec![vec![]]))),
        (a, b, None),
        (a, c, None),
        (a, d, None),
        (a, e, None),
        (a, f, None),
        (a, g, None),
        (a, h, None),
        // b
        (b, a, Some((Milliseconds::new(120), vec![vec![b, a]]))),
        (b, b, Some((Milliseconds::new(0), vec![vec![]]))),
        (b, c, Some((Milliseconds::new(120), vec![vec![b, c]]))),
        (b, d, None),
        (b, e, None),
        (b, f, None),
        (b, g, None),
        (b, h, None),
        // c
        (c, a, Some((Milliseconds::new(120), vec![vec![c, a]]))),
        (c, b, Some((Milliseconds::new(120), vec![vec![c, b]]))),
        (c, c, Some((Milliseconds::new(0), vec![vec![]]))),
        (c, d, None),
        (c, e, None),
        (c, f, None),
        (c, g, None),
        (c, h, None),
        // d
        (d, a, Some((Milliseconds::new(240), vec![vec![d, b, a]]))),
        (d, b, Some((Milliseconds::new(120), vec![vec![d, b]]))),
        (d, c, Some((Milliseconds::new(240), vec![vec![d, b, c]]))),
        (d, d, Some((Milliseconds::new(0), vec![vec![]]))),
        (d, e, Some((Milliseconds::new(240), vec![vec![d, e]]))),
        (d, f, Some((Milliseconds::new(240), vec![vec![d, h, f]]))),
        (d, g, None),
        (d, h, Some((Milliseconds::new(120), vec![vec![d, h]]))),
        // e
        (e, a, Some((Milliseconds::new(480), vec![vec![e, d, b, a]]))),
        (e, b, Some((Milliseconds::new(360), vec![vec![e, d, b]]))),
        (e, c, Some((Milliseconds::new(480), vec![vec![e, d, b, c]]))),
        (e, d, Some((Milliseconds::new(240), vec![vec![e, d]]))),
        (e, e, Some((Milliseconds::new(0), vec![vec![]]))),
        (e, f, Some((Milliseconds::new(120), vec![vec![e, f]]))),
        (e, g, None),
        (e, h, Some((Milliseconds::new(240), vec![vec![e, f, h]]))),
        // f
        (
            f,
            a,
            Some((Milliseconds::new(480), vec![vec![f, h, d, b, a]])),
        ),
        (f, b, Some((Milliseconds::new(360), vec![vec![f, h, d, b]]))),
        (
            f,
            c,
            Some((Milliseconds::new(480), vec![vec![f, h, d, b, c]])),
        ),
        (f, d, Some((Milliseconds::new(240), vec![vec![f, h, d]]))),
        (f, e, Some((Milliseconds::new(120), vec![vec![f, e]]))),
        (f, f, Some((Milliseconds::new(0), vec![vec![]]))),
        (f, g, None),
        (f, h, Some((Milliseconds::new(120), vec![vec![f, h]]))),
        // g
        (
            g,
            a,
            Some((
                Milliseconds::new(600),
                vec![vec![g, f, h, d, b, a], vec![g, e, d, b, a]],
            )),
        ),
        (
            g,
            b,
            Some((
                Milliseconds::new(480),
                vec![vec![g, e, d, b], vec![g, f, h, d, b]],
            )),
        ),
        (
            g,
            c,
            Some((
                Milliseconds::new(600),
                vec![vec![g, e, d, b, c], vec![g, f, h, d, b, c]],
            )),
        ),
        (
            g,
            d,
            Some((
                Milliseconds::new(360),
                vec![vec![g, e, d], vec![g, f, h, d]],
            )),
        ),
        (g, e, Some((Milliseconds::new(120), vec![vec![g, e]]))),
        (g, f, Some((Milliseconds::new(120), vec![vec![g, f]]))),
        (g, g, Some((Milliseconds::new(0), vec![vec![]]))),
        (g, h, Some((Milliseconds::new(240), vec![vec![g, f, h]]))),
        // h
        (h, a, Some((Milliseconds::new(360), vec![vec![h, d, b, a]]))),
        (h, b, Some((Milliseconds::new(240), vec![vec![h, d, b]]))),
        (h, c, Some((Milliseconds::new(360), vec![vec![h, d, b, c]]))),
        (h, d, Some((Milliseconds::new(120), vec![vec![h, d]]))),
        (h, e, Some((Milliseconds::new(240), vec![vec![h, f, e]]))),
        (h, f, Some((Milliseconds::new(120), vec![vec![h, f]]))),
        (h, g, None),
        (h, h, Some((Milliseconds::new(0), vec![vec![]]))),
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
        (ll, ll, Some((Milliseconds::new(0), vec![vec![]]))),
        (ll, bb, Some((Milliseconds::new(600), vec![vec![ll, bb]]))),
        (
            ll,
            rr,
            Some((Milliseconds::new(1080), vec![vec![ll, tl, tr, rr]])),
        ),
        (
            ll,
            tr,
            Some((Milliseconds::new(720), vec![vec![ll, tl, tr]])),
        ),
        (ll, tl, Some((Milliseconds::new(360), vec![vec![ll, tl]]))),
        // bb
        (bb, ll, Some((Milliseconds::new(600), vec![vec![bb, ll]]))),
        (bb, bb, Some((Milliseconds::new(0), vec![vec![]]))),
        (bb, rr, Some((Milliseconds::new(600), vec![vec![bb, rr]]))),
        (
            bb,
            tr,
            Some((Milliseconds::new(960), vec![vec![bb, rr, tr]])),
        ),
        (
            bb,
            tl,
            Some((Milliseconds::new(960), vec![vec![bb, ll, tl]])),
        ),
        // rr
        (
            rr,
            ll,
            Some((Milliseconds::new(1080), vec![vec![rr, tr, tl, ll]])),
        ),
        (rr, bb, Some((Milliseconds::new(600), vec![vec![rr, bb]]))),
        (rr, rr, Some((Milliseconds::new(0), vec![vec![]]))),
        (rr, tr, Some((Milliseconds::new(360), vec![vec![rr, tr]]))),
        (
            rr,
            tl,
            Some((Milliseconds::new(720), vec![vec![rr, tr, tl]])),
        ),
        // tr
        (
            tr,
            ll,
            Some((Milliseconds::new(720), vec![vec![tr, tl, ll]])),
        ),
        (
            tr,
            bb,
            Some((Milliseconds::new(960), vec![vec![tr, rr, bb]])),
        ),
        (tr, rr, Some((Milliseconds::new(360), vec![vec![tr, rr]]))),
        (tr, tr, Some((Milliseconds::new(0), vec![vec![]]))),
        (tr, tl, Some((Milliseconds::new(360), vec![vec![tr, tl]]))),
        // tl
        (tl, ll, Some((Milliseconds::new(360), vec![vec![tl, ll]]))),
        (
            tl,
            bb,
            Some((Milliseconds::new(960), vec![vec![tl, ll, bb]])),
        ),
        (
            tl,
            rr,
            Some((Milliseconds::new(720), vec![vec![tl, tr, rr]])),
        ),
        (tl, tr, Some((Milliseconds::new(360), vec![vec![tl, tr]]))),
        (tl, tl, Some((Milliseconds::new(0), vec![vec![]]))),
    ]
}
