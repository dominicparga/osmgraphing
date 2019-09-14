use super::TestNode;
use osmgraphing::routing;

#[test]
fn simple_stuttgart() {
    let mut astar = routing::factory::new_shortest_path_astar();
    let expected_paths = expected_paths_simple_stuttgart();
    let filepath = "resources/maps/simple_stuttgart.fmi";
    super::assert_correct(&mut astar, expected_paths, filepath);
}

#[test]
fn small() {
    let mut astar = routing::factory::new_shortest_path_astar();
    let expected_paths = expected_paths_small();
    let filepath = "resources/maps/small.fmi";
    super::assert_correct(&mut astar, expected_paths, filepath);
}

//------------------------------------------------------------------------------------------------//

fn expected_paths_simple_stuttgart() -> Vec<(TestNode, TestNode, Option<(u32, Vec<Vec<TestNode>>)>)>
{
    // (idx, id)
    let opp = TestNode::from(0, 26_033_921);
    let bac = TestNode::from(1, 26_160_028);
    let wai = TestNode::from(2, 252_787_940);
    let end = TestNode::from(3, 298_249_467);
    let dea = TestNode::from(4, 1_621_605_361);
    let stu = TestNode::from(5, 2_933_335_353);

    vec![
        // opp
        (opp, opp, Some((0, vec![vec![]]))),
        (opp, bac, Some((8_000, vec![vec![opp, bac]]))),
        (opp, wai, Some((31_000, vec![vec![opp, bac, wai]]))),
        (opp, end, Some((30_000, vec![vec![opp, bac, end]]))),
        (opp, dea, Some((9_069, vec![vec![opp, bac, dea]]))),
        (opp, stu, Some((48_000, vec![vec![opp, bac, wai, stu]]))),
        // bac
        (bac, opp, Some((8_000, vec![vec![bac, opp]]))),
        (bac, bac, Some((0, vec![vec![]]))),
        (bac, wai, Some((23_000, vec![vec![bac, wai]]))),
        (bac, end, Some((22_000, vec![vec![bac, end]]))),
        (bac, dea, Some((1_069, vec![vec![bac, dea]]))),
        (bac, stu, Some((40_000, vec![vec![bac, wai, stu]]))),
        // wai
        (wai, opp, Some((31_000, vec![vec![wai, bac, opp]]))),
        (wai, bac, Some((23_000, vec![vec![wai, bac]]))),
        (wai, wai, Some((0, vec![vec![]]))),
        (wai, end, Some((8_000, vec![vec![wai, end]]))),
        (wai, dea, Some((24_069, vec![vec![wai, bac, dea]]))),
        (wai, stu, Some((17_000, vec![vec![wai, stu]]))),
        // end
        (end, opp, Some((30_000, vec![vec![end, bac, opp]]))),
        (end, bac, Some((22_000, vec![vec![end, bac]]))),
        (end, wai, Some((8_000, vec![vec![end, wai]]))),
        (end, end, Some((0, vec![vec![]]))),
        (end, dea, Some((23_069, vec![vec![end, bac, dea]]))),
        (end, stu, Some((21_000, vec![vec![end, stu]]))),
        // dea
        (dea, opp, None),
        (dea, bac, None),
        (dea, wai, None),
        (dea, end, None),
        (dea, dea, Some((0, vec![vec![]]))),
        (dea, stu, None),
        // stu
        (stu, opp, Some((48_000, vec![vec![stu, wai, bac, opp]]))),
        (stu, bac, Some((40_000, vec![vec![stu, wai, bac]]))),
        (stu, wai, Some((17_000, vec![vec![stu, wai]]))),
        (stu, end, Some((21_000, vec![vec![stu, end]]))),
        (stu, dea, Some((41_069, vec![vec![stu, wai, bac, dea]]))),
        (stu, stu, Some((0, vec![vec![]]))),
    ]
}

fn expected_paths_small() -> Vec<(TestNode, TestNode, Option<(u32, Vec<Vec<TestNode>>)>)> {
    // (idx, id)
    let a = TestNode::from(0, 0);
    let b = TestNode::from(1, 1);
    let c = TestNode::from(2, 2);
    let d = TestNode::from(3, 3);
    let e = TestNode::from(4, 4);
    let f = TestNode::from(5, 5);
    let g = TestNode::from(6, 6);
    let h = TestNode::from(7, 7);

    vec![
        // a
        (a, a, Some((0, vec![vec![]]))),
        (a, b, None),
        (a, c, None),
        (a, d, None),
        (a, e, None),
        (a, f, None),
        (a, g, None),
        (a, h, None),
        // b
        (b, a, Some((1, vec![vec![b, a]]))),
        (b, b, Some((0, vec![vec![]]))),
        (b, c, Some((1, vec![vec![b, c]]))),
        (b, d, None),
        (b, e, None),
        (b, f, None),
        (b, g, None),
        (b, h, None),
        // c
        (c, a, Some((1, vec![vec![c, a]]))),
        (c, b, Some((1, vec![vec![c, b]]))),
        (c, c, Some((0, vec![vec![]]))),
        (c, d, None),
        (c, e, None),
        (c, f, None),
        (c, g, None),
        (c, h, None),
        // d
        (d, a, Some((2, vec![vec![d, b, a]]))),
        (d, b, Some((1, vec![vec![d, b]]))),
        (d, c, Some((2, vec![vec![d, b, c]]))),
        (d, d, Some((0, vec![vec![]]))),
        (d, e, Some((2, vec![vec![d, e]]))),
        (d, f, Some((2, vec![vec![d, h, f]]))),
        (d, g, None),
        (d, h, Some((1, vec![vec![d, h]]))),
        // e
        (e, a, Some((4, vec![vec![e, d, b, a]]))),
        (e, b, Some((3, vec![vec![e, d, b]]))),
        (e, c, Some((4, vec![vec![e, d, b, c]]))),
        (e, d, Some((2, vec![vec![e, d]]))),
        (e, e, Some((0, vec![vec![]]))),
        (e, f, Some((1, vec![vec![e, f]]))),
        (e, g, None),
        (e, h, Some((2, vec![vec![e, f, h]]))),
        // f
        (f, a, Some((4, vec![vec![f, h, d, b, a]]))),
        (f, b, Some((3, vec![vec![f, h, d, b]]))),
        (f, c, Some((4, vec![vec![f, h, d, b, c]]))),
        (f, d, Some((2, vec![vec![f, h, d]]))),
        (f, e, Some((1, vec![vec![f, e]]))),
        (f, f, Some((0, vec![vec![]]))),
        (f, g, None),
        (f, h, Some((1, vec![vec![f, h]]))),
        // g
        (g, a, Some((5, vec![vec![g, e, d, b, a]]))),
        (g, b, Some((4, vec![vec![g, e, d, b]]))),
        (g, c, Some((5, vec![vec![g, e, d, b, c]]))),
        (g, d, Some((3, vec![vec![g, e, d], vec![g, f, d]]))),
        (g, e, Some((1, vec![vec![g, e]]))),
        (g, f, Some((1, vec![vec![g, f]]))),
        (g, g, Some((0, vec![vec![]]))),
        (g, h, Some((2, vec![vec![g, f, h]]))),
        // h
        (h, a, Some((3, vec![vec![h, d, b, a]]))),
        (h, b, Some((2, vec![vec![h, d, b]]))),
        (h, c, Some((3, vec![vec![h, d, b, c]]))),
        (h, d, Some((1, vec![vec![h, d]]))),
        (h, e, Some((2, vec![vec![h, f, e]]))),
        (h, f, Some((1, vec![vec![h, f]]))),
        (h, g, None),
        (h, h, Some((0, vec![vec![]]))),
    ]
}
