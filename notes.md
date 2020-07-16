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
- Use population-data to get routes
  - Maybe use [realistic src-dst-routes][acm/micro-travel-demand] ([GitHub-repo][github/vbuchhold/routing-framework])
  - Set population of specific spots and interpolate somehow
    - [worldometers][worldometers/germany]
    - [German Federal Statistical Office][destatis]
      - [different population-data of Baden-Württemberg][statistik-bw]
      - [different population-data of Germany, but per mini-square][atlas.zensus2011.de]
        - Zensus: `10 %` of population every 10 years
        - Mikrozensus: `1 %` of population every year
      - [area-statistics (e.g. traffic-area in percent)][statistikportal]
    - Get population from osm-data
      - Take nodes/ways and distribute according to max-speed (low speed <-> high population-density).
      - Take city-level and let routes go from lower to higher levels.
- Reduce visibility of modules and control public-access, e.g. of module `defaults`, which is only needed in tests.
- Mention and explain cargo-features somewhere.


### Build-script

\-


### Documentation

- Write down alternative implementation-approaches
  - Routing from src-node to dst-node where dst-node has at least the city-level of the src-node.


### Extend tests

- extend routing-tests
  - implement routing-tests for parsed pbf-files (just comparing src-id, dst-id and distance)
- Take results from actions of commit f28d88a for parsing-tests (fmi).
- Test personalized routing explicitly using certain alpha-values and new expected paths.
- How to test exploration?
  - Create graph of 2 nodes and 8 edges, where 3 edges are dominated by the others.
  - At least `2d + 1` edges are needed.
  - Test restriction(?)
- Test created route-files.
- Test edge-ids (especially parsing).
- Write config-tests without checking content, so configs can be checked automatically.


### Extend configs

- Warn user when parsing `pbf`-map, if unused categories are provided in the config.
- Write parser __parsing all configs__ at once.
- Cleanup `kebab-cases` and `snake_cases` etc.
- Check if writing-cfg contains shortcut-indices when `is_ch-graph == false`.
- Some configs are damaged!


### Extend parsing

- Use __preprocessing-phase__ for `pbf`-parser to count edges and __allocate memory__ accordingly.
- Print __edit-link__ for weird osm-content (in addition to currently printed warnings).
- __Parse lanes__ (currently, default is used).
  - tags: `lanes`, `lanes:backward` (`way-id: 33172848`)


### Extend routing

\-


### Extend balancing

- Update route-counts of shortcuts after updating normal edges.
- Flatten the found routes after the loops and cumulate all workloads for sc-edges at once.
  This reduces the access to edges.
- Use `ch-constructor` (written in `c/cpp`) as binary or build wrapping rust-crate?


## Info

- [C with Rust][rust-docs/c-with-rust]
- [OSM-tags][taginfo]
- serde-yaml
  - `https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust`
  - `https://serde.rs/attributes.html`
  - `https://serde.rs/container-attrs.html`
  - `https://serde.rs/variant-attrs.html`
  - `https://serde.rs/field-attrs.html`
  - `https://serde.rs/enum-representations.html`
  - `https://docs.rs/serde_yaml/0.8.11/serde_yaml/`
- Mapviewer-libs
  - actix-web (Rust)
  - [leafletjs (JavaScript)][leafletjs]
  - [Marble (C++ or python)][kde/marble]
  - [JMapViewer (Java)][osm/wiki/jmapviewer]


### License

[When is a program and its plug-ins considered a single combined program?][gnu/licenses/gpl-faq/gplplugins]


[acm/micro-travel-demand]: https://dl.acm.org/doi/10.1145/3347146.3359361
[atlas.zensus2011.de]: https://atlas.zensus2011.de/
[destatis]: https://www.destatis.de/DE/Service/Statistik-Visualisiert/RegionalatlasAktuell.html
[github/vbuchhold/routing-framework]: https://github.com/vbuchhold/routing-framework
[gnu/licenses/gpl-faq/gplplugins]: https://www.gnu.org/licenses/gpl-faq.html#GPLPlugins
[kde/marble]: http://api.kde.org/4.x-api/kdeedu-apidocs/marble/html/namespaceMarble.html
[leafletjs]: https://leafletjs.com/
[osm/wiki/jmapviewer]: https://wiki.openstreetmap.org/wiki/JMapViewer
[rust-docs/c-with-rust]: https://rust-embedded.github.io/book/interoperability/c-with-rust.html
[statistik-bw]: https://www.statistik-bw.de/Intermaptiv/?re=gemeinde&ags=08317057&i=01202&r=0&g=0001&afk=5&fkt=besetzung&fko=mittel
[statistikportal]: https://www.statistikportal.de/de/flaechenatlas
[taginfo]: https://taginfo.openstreetmap.org/
[worldometers/germany]: https://www.worldometers.info/world-population/germany-population/