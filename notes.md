# Notes

Just a place to store arbitrary information, TODOs and so on.
Maybe, it is concept for later documentation, or just keep-up-the-good-work (`kutgw`).


## Ideas

### General

- Implement graph as server, which can execute queries from clients (e.g. via channels).
- Building needs much more memory for `Germany.pbf` (~ `14 GB`) due to sc-edges and meta-info.
  When creating metrics, memory-consumption shrinks to `10/11 GB` and lower, probably because these values are released.
  It could make sense to implement simple (de-)serialization for the graph (`map-file.rfmi`, standing for `raw fmi`).
- Write __working-off chunks__ in builder in separate function using `From<Edge>` or `Into<Edge>`


### Build-script

- A build-script could, maybe, build the inline-size for `SmallVec` dependent of an env-var when compiling.
  The command `include!(...)` could help.
  More info in [this cargo-issue][github/rust-lang/cargo/issues/5624].


### Documentation

- Write down alternative implementation-approaches
  - Routing from src-node to dst-node where dst-node has at least the city-level of the src-node.


### Extend tests

- extend routing-tests
  - implement routing-tests for parsed pbf-files (just comparing src-id, dst-id and distance)
- Take results from actions of commit f28d88a for parsing-tests (fmi).
- Test personalized routing explicitly using certain alpha-values and new expected paths.


### Extend configs

- Warn user when parsing `pbf`-map, if unused categories are provided in the config.


### Extend parsing

- Use __preprocessing-phase__ for `pbf`-parser to count edges and __allocate memory__ accordingly.
- Print __edit-link__ for weird osm-content (in addition to currently printed warnings).
- Parse lanes (currently, default is used).
  - tags: `lanes`, `lanes:backward` (`way-id: 33172848`)


### Extend generating

- Process `parser::EdgeCategory::ShortcutEdgeIdx`


### Extend routing

- Implement little parser for a file containing routes.
  Preferred is a format, where every line defines `src, dst`.
  To make this less dependent of a certain map, every node is represented by its coordinate or id instead of its index.


## serde-yaml-Info

- `https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust`
- `https://serde.rs/attributes.html`
- `https://serde.rs/container-attrs.html`
- `https://serde.rs/variant-attrs.html`
- `https://serde.rs/field-attrs.html`
- `https://serde.rs/enum-representations.html`
- `https://docs.rs/serde_yaml/0.8.11/serde_yaml/`


## Mapviewer-libs

- actix-web (Rust)
- [leafletjs (JavaScript)][leafletjs]
- [Marble (C++ or python)][kde/marble]
- [JMapViewer (Java)][osm/wiki/jmapviewer]


## Proof of correctness for bidirectional Dijkstra

The termination of the bidirectional Astar is based on the first node v, that is marked by both, the forward- and the backward-subroutine.
However, this common node v is part of the shortest path s->t wrt to this particular hop-distance H, but doesn't have to be part of the shortest path s->t wrt to edge-weights.

Every node, that is not settled in any of the both subroutines, has a longer distance to both s and t than the already found common node v and hence can not be part of the shortest path (wrt to edge-weights).
Otherwise, it would have been settled before v since the priority-queues sort by weights.
In other words, only already settled nodes and their neighbors (which are already enqueued) can be part of the shortest path.

In conclusion, emptying the remaining nodes in the queues and picking the shortest path of the resulting common nodes leads to the shortest path wrt to edge-weights from s to t.


## Proof of correctness for bidirectional Dijkstra for contracted graphs

Here, the proof for bidirectional Dijkstra doesn't hold, because each sub-graph doesn't visit every node of the total graph, due to the level-filter when pushing edges to the queue.
Hence, the forward- and the backward-query are not balanced wrt weights.
Thus, after finding the first meeting-node, the hop-distance of the shortest-path could be arbitrary.
This leads to wrong paths with normal bidirectional Dijkstra.
To correct this issue, stop the query after polling a node of a sub-distance, which is higher than the currently best meeting-node's total distance.


[github/rust-lang/cargo/issues/5624]: https://github.com/rust-lang/cargo/issues/5624
[kde/marble]: http://api.kde.org/4.x-api/kdeedu-apidocs/marble/html/namespaceMarble.html
[leafletjs]: https://leafletjs.com/
[osm/wiki/jmapviewer]: https://wiki.openstreetmap.org/wiki/JMapViewer
