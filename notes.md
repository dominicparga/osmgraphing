# Notes

Just a place to store arbitrary information, TODOs and so on.
Maybe, it is concept for later documentation, or just keep-up-the-good-work (`kutgw`).


## Mapviewer-libs

- actix-web (Rust)
- [leafletjs (JavaScript)][leafletjs]
- [Marble (C++ or python)][kde/marble]
- [JMapViewer (Java)][osm/wiki/jmapviewer]


## Ideas

- Use log::debug by flag
- implement tests comparing upcoming ch-dijkstra with normal dijkstra on isle-of-man
- implement routing-tests for parsed pbf-files (just comparing src-id, dst-id and length)
- flag: keep metric or not -> save memory
- Write down alternative implementation-approaches
  - Implement shortcut-edges more memory-efficient storing list of costs per src-dst-pair instead of per edge.
  - Routing from src-node to dst-node where dst-node has at least the city-level of the src-node.
- Use preprocessing-phase for `pbf`-parser to count edges and allocate memory accordingly.
- Implement `config-settings` for nodes (e.g. coordinates in float vs unsigned integral)
- Replace duration in milliseconds by seconds.


[kde/marble]: http://api.kde.org/4.x-api/kdeedu-apidocs/marble/html/namespaceMarble.html
[leafletjs]: https://leafletjs.com/
[osm/wiki/jmapviewer]: https://wiki.openstreetmap.org/wiki/JMapViewer
