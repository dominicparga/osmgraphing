use super::TestNode;
use osmgraphing::{network::NodeIdx, routing, units::time::Milliseconds};

#[test]
fn simple_stuttgart() {
    let mut dijkstra = routing::factory::dijkstra::fastest();
    let expected_paths = expected_paths_simple_stuttgart();
    let filepath = "resources/maps/simple_stuttgart.fmi";
    super::assert_correct(&mut dijkstra, expected_paths, filepath);
}

#[test]
fn small() {
    let mut dijkstra = routing::factory::dijkstra::fastest();
    let expected_paths = expected_paths_small();
    let filepath = "resources/maps/small.fmi";
    super::assert_correct(&mut dijkstra, expected_paths, filepath);
}

//------------------------------------------------------------------------------------------------//

fn expected_paths_simple_stuttgart() -> Vec<(
    TestNode,
    TestNode,
    Option<(Milliseconds, Vec<Vec<TestNode>>)>,
)> {
    // (idx, id)
    let opp = TestNode::from(NodeIdx::new(0), 26_033_921);
    let bac = TestNode::from(NodeIdx::new(1), 26_160_028);
    let wai = TestNode::from(NodeIdx::new(2), 252_787_940);
    let end = TestNode::from(NodeIdx::new(3), 298_249_467);
    let dea = TestNode::from(NodeIdx::new(4), 1_621_605_361);
    let stu = TestNode::from(NodeIdx::new(5), 2_933_335_353);

    vec![
        // opp
        (opp, opp, Some((0.into(), vec![vec![]]))),
        (opp, bac, Some((576_000.into(), vec![vec![opp, bac]]))),
        (
            opp,
            wai,
            Some((1_266_000.into(), vec![vec![opp, bac, wai]])),
        ),
        (
            opp,
            end,
            Some((1_566_000.into(), vec![vec![opp, bac, end]])),
        ),
        (opp, dea, Some((704_280.into(), vec![vec![opp, bac, dea]]))),
        (
            opp,
            stu,
            Some((1_878_000.into(), vec![vec![opp, bac, wai, stu]])),
        ),
        // bac
        (bac, opp, Some((576_000.into(), vec![vec![bac, opp]]))),
        (bac, bac, Some((0.into(), vec![vec![]]))),
        (bac, wai, Some((690_000.into(), vec![vec![bac, wai]]))),
        (bac, end, Some((990_000.into(), vec![vec![bac, end]]))),
        (bac, dea, Some((128_280.into(), vec![vec![bac, dea]]))),
        (
            bac,
            stu,
            Some((1_302_000.into(), vec![vec![bac, wai, stu]])),
        ),
        // wai
        (
            wai,
            opp,
            Some((1_266_000.into(), vec![vec![wai, bac, opp]])),
        ),
        (wai, bac, Some((690_000.into(), vec![vec![wai, bac]]))),
        (wai, wai, Some((0.into(), vec![vec![]]))),
        (wai, end, Some((576_000.into(), vec![vec![wai, end]]))),
        (wai, dea, Some((818_280.into(), vec![vec![wai, bac, dea]]))),
        (wai, stu, Some((612_000.into(), vec![vec![wai, stu]]))),
        // end
        (
            end,
            opp,
            Some((1_566_000.into(), vec![vec![end, bac, opp]])),
        ),
        (end, bac, Some((990_000.into(), vec![vec![end, bac]]))),
        (end, wai, Some((576_000.into(), vec![vec![end, wai]]))),
        (end, end, Some((0.into(), vec![vec![]]))),
        (
            end,
            dea,
            Some((1_118_280.into(), vec![vec![end, bac, dea]])),
        ),
        (end, stu, Some((945_000.into(), vec![vec![end, stu]]))),
        // dea
        (dea, opp, None),
        (dea, bac, None),
        (dea, wai, None),
        (dea, end, None),
        (dea, dea, Some((0.into(), vec![vec![]]))),
        (dea, stu, None),
        // stu
        (
            stu,
            opp,
            Some((1_878_000.into(), vec![vec![stu, wai, bac, opp]])),
        ),
        (
            stu,
            bac,
            Some((1_302_000.into(), vec![vec![stu, wai, bac]])),
        ),
        (stu, wai, Some((612_000.into(), vec![vec![stu, wai]]))),
        (stu, end, Some((945_000.into(), vec![vec![stu, end]]))),
        (
            stu,
            dea,
            Some((1_430_280.into(), vec![vec![stu, wai, bac, dea]])),
        ),
        (stu, stu, Some((0.into(), vec![vec![]]))),
    ]
}

fn expected_paths_small() -> Vec<(
    TestNode,
    TestNode,
    Option<(Milliseconds, Vec<Vec<TestNode>>)>,
)> {
    // (idx, id)
    let a = TestNode::from(NodeIdx::new(0), 0);
    let b = TestNode::from(NodeIdx::new(1), 1);
    let c = TestNode::from(NodeIdx::new(2), 2);
    let d = TestNode::from(NodeIdx::new(3), 3);
    let e = TestNode::from(NodeIdx::new(4), 4);
    let f = TestNode::from(NodeIdx::new(5), 5);
    let g = TestNode::from(NodeIdx::new(6), 6);
    let h = TestNode::from(NodeIdx::new(7), 7);

    vec![
        // a
        (a, a, Some((0.into(), vec![vec![]]))),
        (a, b, None),
        (a, c, None),
        (a, d, None),
        (a, e, None),
        (a, f, None),
        (a, g, None),
        (a, h, None),
        // b
        (b, a, Some((120.into(), vec![vec![b, a]]))),
        (b, b, Some((0.into(), vec![vec![]]))),
        (b, c, Some((120.into(), vec![vec![b, c]]))),
        (b, d, None),
        (b, e, None),
        (b, f, None),
        (b, g, None),
        (b, h, None),
        // c
        (c, a, Some((120.into(), vec![vec![c, a]]))),
        (c, b, Some((120.into(), vec![vec![c, b]]))),
        (c, c, Some((0.into(), vec![vec![]]))),
        (c, d, None),
        (c, e, None),
        (c, f, None),
        (c, g, None),
        (c, h, None),
        // d
        (d, a, Some((240.into(), vec![vec![d, b, a]]))),
        (d, b, Some((120.into(), vec![vec![d, b]]))),
        (d, c, Some((240.into(), vec![vec![d, b, c]]))),
        (d, d, Some((0.into(), vec![vec![]]))),
        (d, e, Some((240.into(), vec![vec![d, e]]))),
        (d, f, Some((240.into(), vec![vec![d, h, f]]))),
        (d, g, None),
        (d, h, Some((120.into(), vec![vec![d, h]]))),
        // e
        (e, a, Some((480.into(), vec![vec![e, d, b, a]]))),
        (e, b, Some((360.into(), vec![vec![e, d, b]]))),
        (e, c, Some((480.into(), vec![vec![e, d, b, c]]))),
        (e, d, Some((240.into(), vec![vec![e, d]]))),
        (e, e, Some((0.into(), vec![vec![]]))),
        (e, f, Some((120.into(), vec![vec![e, f]]))),
        (e, g, None),
        (e, h, Some((240.into(), vec![vec![e, f, h]]))),
        // f
        (f, a, Some((480.into(), vec![vec![f, h, d, b, a]]))),
        (f, b, Some((360.into(), vec![vec![f, h, d, b]]))),
        (f, c, Some((480.into(), vec![vec![f, h, d, b, c]]))),
        (f, d, Some((240.into(), vec![vec![f, h, d]]))),
        (f, e, Some((120.into(), vec![vec![f, e]]))),
        (f, f, Some((0.into(), vec![vec![]]))),
        (f, g, None),
        (f, h, Some((120.into(), vec![vec![f, h]]))),
        // g
        (
            g,
            a,
            Some((
                600.into(),
                vec![vec![g, f, h, d, b, a], vec![g, e, d, b, a]],
            )),
        ),
        (
            g,
            b,
            Some((480.into(), vec![vec![g, e, d, b], vec![g, f, h, d, b]])),
        ),
        (
            g,
            c,
            Some((
                600.into(),
                vec![vec![g, e, d, b, c], vec![g, f, h, d, b, c]],
            )),
        ),
        (
            g,
            d,
            Some((360.into(), vec![vec![g, e, d], vec![g, f, h, d]])),
        ),
        (g, e, Some((120.into(), vec![vec![g, e]]))),
        (g, f, Some((120.into(), vec![vec![g, f]]))),
        (g, g, Some((0.into(), vec![vec![]]))),
        (g, h, Some((240.into(), vec![vec![g, f, h]]))),
        // h
        (h, a, Some((360.into(), vec![vec![h, d, b, a]]))),
        (h, b, Some((240.into(), vec![vec![h, d, b]]))),
        (h, c, Some((360.into(), vec![vec![h, d, b, c]]))),
        (h, d, Some((120.into(), vec![vec![h, d]]))),
        (h, e, Some((240.into(), vec![vec![h, f, e]]))),
        (h, f, Some((120.into(), vec![vec![h, f]]))),
        (h, g, None),
        (h, h, Some((0.into(), vec![vec![]]))),
    ]
}
