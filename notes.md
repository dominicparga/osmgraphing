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


## Multiple metrices, where the amount is known at runtime

Use a `MetricIdx` and a `Vec<Vec<MetricU32>>` to access metrices.
Via yaml-config, the parser can be adjusted to parse (edge-)metrics in `fmi`-files in the right order (as shown below) and ignore columns.
Further, the default-preference of the personalized-routing can be set with weights.
The example below shows a case, where the metric `length` is weighted with `169 / 500 = 33.8 %` while the metric `duration` is weighted with `331 / 500 = 66.2 %`.

```yaml
graph:
  metrics:
  - id: "length"
    type: meters
  - id: "maxspeed"
    type: kmph
  - type: ignore
  - id: "duration"
    type: milliseconds
  - id: "lane-count"
    type: u8
  - id: "custom"
    type: u32

routing:
  preferences:
  - id: "length"
    alpha: 169
  - id: "duration"
    alpha: 331
```


## Other

- Write down alternative implementation-approaches
  - Building the graph could be improved in memory-usage by processing edge-packets and replacing them instead of processing all at once.
  - Implement shortcut-edges more memory-efficient storing list of costs per src-dst-pair instead of per edge.
  - Routing from src-node to dst-node where dst-node has at least the city-level of the src-node.
