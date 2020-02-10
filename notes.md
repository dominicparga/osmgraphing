# Notes

Just a place to store arbitrary information, TODOs and so on.
Maybe, it is concept for later documentation, or just keep-up-the-good-work (`kutgw`).


## Mapviewer-libs

- actix-web (Rust)
- [leafletjs (JavaScript)][leafletjs]
- [Marble (C++ or python)][kde/marble]
- [JMapViewer (Java)][osm/wiki/jmapviewer]


[kde/marble]: http://api.kde.org/4.x-api/kdeedu-apidocs/marble/html/namespaceMarble.html
[leafletjs]: https://leafletjs.com/
[osm/wiki/jmapviewer]: https://wiki.openstreetmap.org/wiki/JMapViewer


## Other

- Write down alternative implementation-approaches
- Building the graph could be improved in memory-usage by processing edge-packets and replacing them instead of processing all at once.
- Implement shortcut-edges more memory-efficient storing list of costs per src-dst-pair instead of per edge.
- Routing from src-node to dst-node where dst-node has at least the city-level of the src-node.
