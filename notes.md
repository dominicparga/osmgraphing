# Notes

Just a place to store arbitrary information, TODOs and so on.
Maybe, it is concept for later documentation, or just keep-up-the-good-work (`kutgw`).


## Ideas

### General

- Update README (`"Memory-usage and performance have been better, but now, the graph supports multiple metrics."` -> it's great again)
- Use log::debug by flag


### Documentation

- Write down alternative implementation-approaches
  - Implement shortcut-edges more memory-efficient storing list of costs per src-dst-pair instead of per edge.
  - Routing from src-node to dst-node where dst-node has at least the city-level of the src-node.
- extend configs
  - flag: keep metric or not -> save memory
    NO because graph is a static thing.


### Extend tests

- extend routing-tests
  - implement tests comparing upcoming ch-dijkstra with normal dijkstra on isle-of-man
  - implement routing-tests for parsed pbf-files (just comparing src-id, dst-id and length)
- Take results from actions of commit f28d88a for parsing-tests (fmi).


### Extend configs

- Implement `config-settings` for nodes (e.g. coordinates in float vs unsigned integral)


### Extend parsing

- Use preprocessing-phase for `pbf`-parser to count edges and allocate memory accordingly.


## Mapviewer-libs

- actix-web (Rust)
- [leafletjs (JavaScript)][leafletjs]
- [Marble (C++ or python)][kde/marble]
- [JMapViewer (Java)][osm/wiki/jmapviewer]


## serde-yaml-Info

- `https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust`
- `https://serde.rs/attributes.html`
- `https://serde.rs/container-attrs.html`
- `https://serde.rs/variant-attrs.html`
- `https://serde.rs/field-attrs.html`
- `https://serde.rs/enum-representations.html`
- `https://docs.rs/serde_yaml/0.8.11/serde_yaml/`


[kde/marble]: http://api.kde.org/4.x-api/kdeedu-apidocs/marble/html/namespaceMarble.html
[leafletjs]: https://leafletjs.com/
[osm/wiki/jmapviewer]: https://wiki.openstreetmap.org/wiki/JMapViewer
