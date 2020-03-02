use super::super::TestType;

pub fn isle_of_man() {
    let cfg = super::create_config(TestType::IsleOfMan);
    let graph = super::parse(cfg.graph);

    let nodes = graph.nodes();
    let expected = 51_310;
    assert_eq!(
        nodes.count(),
        expected,
        "Number of nodes in graph should be {} but is {}.",
        expected,
        nodes.count()
    );
    let fwd_edges = graph.fwd_edges();
    let expected = 103_920;
    assert_eq!(
        fwd_edges.count(),
        expected,
        "Number of fwd-edges in graph should be {} but is {}.",
        expected,
        fwd_edges.count()
    );
}
