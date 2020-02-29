# Notes

Just a place to store arbitrary information, TODOs and so on.
Maybe, it is concept for later documentation, or just keep-up-the-good-work (`kutgw`).


## Mapviewer-libs

- actix-web (Rust)
- [leafletjs (JavaScript)][leafletjs]
- [Marble (C++ or python)][kde/marble]
- [JMapViewer (Java)][osm/wiki/jmapviewer]


## Next goals

- Use log::debug by flag
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
- update config-documentation

  ```yaml
  # old version

  graph:
    map-file: "resources/maps/small.fmi"
    vehicles:
      type: Car
      is-nice-to-use: false
    edges:
      metrics:
      - type: ignore
        id: src-id
      - type: ignore
        id: dst-id
      - type: length
        provided: false
      - type: maxspeed
        provided: true
      - type: duration
        provided: false
      - type: lane-count

  routing: # example with two metrics and weights
    metrics: [length, duration]
    preferences:
    - id: length
      alpha: 169
    - id: duration
      alpha: 331
  ```

- Update Changelog: Cleanup parser and make pbf-parser single-threaded, since new bottleneck is memory-usage.
  The runtime for parsing ways single-threaded and multi-threaded was
  identical (3:30 minutes for multi-threaded, 3:20 minutes for
  single-threaded).
  The times are much faster, if the RAM has remaining capacity and doesn't have to use the swap-partition.


## Other

- Write down alternative implementation-approaches
  - Implement shortcut-edges more memory-efficient storing list of costs per src-dst-pair instead of per edge.
  - Routing from src-node to dst-node where dst-node has at least the city-level of the src-node.


[kde/marble]: http://api.kde.org/4.x-api/kdeedu-apidocs/marble/html/namespaceMarble.html
[leafletjs]: https://leafletjs.com/
[osm/wiki/jmapviewer]: https://wiki.openstreetmap.org/wiki/JMapViewer
