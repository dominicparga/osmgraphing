# Notes

Just a place to store arbitrary information, TODOs and so on.
Maybe, it is concept for later documentation, or just keep-up-the-good-work (`kutgw`).


## Mapviewer-libs

- actix-web (Rust)
- [leafletjs (JavaScript)][leafletjs]
- [Marble (C++ or python)][kde/marble]
- [JMapViewer (Java)][osm/wiki/jmapviewer]


## Next goals

- store `Vec<Vec<u32>>` in `Graph` instead of several metrics
  - __*done*__ with same runtime for routing `:)`, but doubled runtime for parsing `:(`
- if user asks for metric in Meters, just give it
  - __*done*__ (e.g. `length(metric_idx)` returns as `Meters`, while `metric(metric_idx)` returns as `MetricU32`)
- helper-methods, like scalar-product
- implement tests comparing upcoming ch-dijkstra with normal dijkstra on isle-of-man
- implement routing-tests for parsed pbf-files (just comparing src-id, dst-id and length)
- flag: keep metric or not
- parse config-string into config
- parse config-yaml into config


## Other

- Write down alternative implementation-approaches
  - Implement shortcut-edges more memory-efficient storing list of costs per src-dst-pair instead of per edge.
  - Routing from src-node to dst-node where dst-node has at least the city-level of the src-node.


[kde/marble]: http://api.kde.org/4.x-api/kdeedu-apidocs/marble/html/namespaceMarble.html
[leafletjs]: https://leafletjs.com/
[osm/wiki/jmapviewer]: https://wiki.openstreetmap.org/wiki/JMapViewer
