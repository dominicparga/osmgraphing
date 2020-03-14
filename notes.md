# Notes

Just a place to store arbitrary information, TODOs and so on.
Maybe, it is concept for later documentation, or just keep-up-the-good-work (`kutgw`).


## Ideas

### General

- Implement graph as server, which can execute queries from clients (e.g. via channels).


### Build-script

- A build-script could, maybe, build the inline-size for `SmallVec` dependent of an env-var when compiling.
  The command `include!(...)` could help.
  More info in [this cargo-issue][github/rust-lang/cargo/issues/5624].


### Documentation

- Write down alternative implementation-approaches
  - Routing from src-node to dst-node where dst-node has at least the city-level of the src-node.
  - Implement shortcuts with array `[edge: EdgeIdx -> is_sc: bool]` and array `[some_idx: usize -> (edge, sc0, sc1): (EdgeIdx, EdgeIdx, EdgeIdx)]`, latter sorted by edge to search logarithmically.


### Extend tests

- extend routing-tests
  - implement tests comparing ch-dijkstra with normal dijkstra on isle-of-man
  - implement routing-tests for parsed pbf-files (just comparing src-id, dst-id and length)
- Take results from actions of commit f28d88a for parsing-tests (fmi).
- Test personalized routing explicitly using certain alpha-values and new expected paths.


### Extend configs

\-


### Extend parsing

- Use __preprocessing-phase__ for `pbf`-parser to count edges and __allocate memory__ accordingly.
- Print __edit-link__ for weird osm-content (in addition to currently printed warnings).
- Parse lanes (now: use default).
  - tags: `lanes`, `lanes:backward` (`way-id: 33172848`)


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


[github/rust-lang/cargo/issues/5624]: https://github.com/rust-lang/cargo/issues/5624
[kde/marble]: http://api.kde.org/4.x-api/kdeedu-apidocs/marble/html/namespaceMarble.html
[leafletjs]: https://leafletjs.com/
[osm/wiki/jmapviewer]: https://wiki.openstreetmap.org/wiki/JMapViewer
