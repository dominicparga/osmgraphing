# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this project adheres to [Semantic Versioning][semver].


## Table of contents

1. [Unreleased](#unreleased)
1. [v0.11.0](#v0.11.0)
1. [v0.10.0](#v0.10.0)
1. [v0.9.0](#v0.9.0)
1. [v0.8.0](#v0.8.0)
1. [v0.7.1](#v0.7.1)
    1. [v0.7.0](#v0.7.0)
1. [v1.0.0-yanked](#v1.0.0)
1. [v0.6.1](#v0.6.1)
    1. [v0.6.0](#v0.6.0)
1. [v0.5.0](#v0.5.0)
1. [v0.4.1](#v0.4.1)
    1. [v0.4.0](#v0.4.0)
1. [v0.3.1](#v0.3.1)
    1. [v0.3.0](#v0.3.0)
1. [v0.2.4](#v0.2.4)
    1. [v0.2.3](#v0.2.3)
    1. [v0.2.2](#v0.2.2)
    1. [v0.2.1](#v0.2.1)
    1. [v0.2.0](#v0.2.0)
1. [v0.1.5](#v0.1.5)
    1. [v0.1.4](#v0.1.4)
    1. [v0.1.3](#v0.1.3)
    1. [v0.1.2](#v0.1.2)
    1. [v0.1.1](#v0.1.1)
    1. [v0.1.0](#v0.1.0)


## [Unreleased][github/self/unreleased] <a name="unreleased"></a>

### Added <a name="unreleased/added"></a>

\-


### Changed <a name="unreleased/changed"></a>

\-


### Deprecated <a name="unreleased/deprecated"></a>

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- The link to `doc.rs` is hardcoded to `major.minor.patch=0.y.z` because `docs.rs` chooses version `1.0.0` though it's yanked..
- Problem: The generator doesn't convert metrics from kilometers in meters, but parser reads only meters.
- Problem: The `fmi`-parser is not checking `is_metric_provided(...)`.
  If the number of `EdgeCategory`s in a config-file for `fmi`-files is higher than the number of elements provided by the `fmi`-parser, the file cannot be parsed.
  This case is relevant since `EdgeCategory`s don't have to be provided by the map-file, but could be calculated.


### Removed <a name="unreleased/removed"></a>

\-


### Fixed <a name="unreleased/fixed"></a>

\-


### Security <a name="unreleased/security"></a>

\-


## [v0.11.0][github/self/v0.11.0] <a name="v0.11.0"></a>

### Added <a name="v0.11.0/added"></a>

- Implement a __`mapgenerator`__ generating `fmi`-map-files by converting from one format to another via config-files.
  - Implement a module `generating`.
  - Generate and test __`isle-of-man.fmi`__ out of `isle-of-man.pbf`.
  - Implement a config for __pbf-to-fmi-conversion__.
- Make __config__ cleaner and more flexible.
  - Add __configs__ for every map, that is being __tested__.
  - Add respective __test-cases__.
  - Add support for `NodeCategory`s and add variants to `EdgeCategory`s.
- Implement a nice trait for __supporting file-extensions__ in a general way.


### Changed <a name="v0.11.0/changed"></a>

- Bring sense to __TOC in `CHANGELOG.md`__.
- Update __`README.md`__.
- Reduce default-inline-size of `SmallVec` from 5 to 4 (`pub const SMALL_VEC_INLINE_SIZE: usize = 4`).
- Move existing module `parsing` and new module `generating` into one module called __`io`__.
- Extend parser's preprocessing by checking config.
- Make __config__ cleaner and more flexible.
  - Refactor config completely by separating strictly between __`raw-config`__ (direct result of deserialization) and the final config-version.
  - Rename __config-attributes__ and __config-methods__.
  - Use `Option<...>` for large config-parts (`parser` vs `generator` vs `routing`).
  - Rename config `schema.yaml` to __`blueprint.yaml`__ to keep the keyword `schema` free.
  - Split __`EdgeCategory::NodeId`__ to `EdgeCategory::SrcId` and `EdgeCategory::DstId`.
  - Let config remember every id (expect for ignore).
- Update `notes.md`.
- Update `ProtoNode`
  - Make graphbuilder's __`ProtoNode`__ public.
  - Let `ProtoNode` remember a `bool` instead of an `edge-count` for the info, whether it is part of an edge.
- Simplify __metric-access__ in graph.


### Deprecated <a name="v0.11.0/deprecated"></a>

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- The link to `doc.rs` is hardcoded to `major.minor.patch=0.y.z` because `docs.rs` chooses version `1.0.0` though it's yanked..
- Problem: The generator doesn't convert metrics from kilometers in meters, but parser reads only meters.
- Problem: The `fmi`-parser is not checking `is_metric_provided(...)`.
  If the number of `EdgeCategory`s in a config-file for `fmi`-files is higher than the number of elements provided by the `fmi`-parser, the file cannot be parsed.
  This case is relevant since `EdgeCategory`s don't have to be provided by the map-file, but could be calculated.


### Removed <a name="v0.11.0/removed"></a>

- Make __config__ cleaner and more flexible.
  - __Simplify tests__ by creating configs directly (instead of using enum `TestType`).


### Fixed <a name="v0.11.0/fixed"></a>

- Let little __build-script__ stop when error occurs.
- Remove use of `mul_add` to allow compiler to optimize loop (`SIMD`).
- Let graphbuilder remove __edge-duplicates__ (same src, same dst and exactly, not approximately, same metric).
  This will also be efficient for shortcuts wrt contraction-hierarchies.


## [v0.10.0][github/self/v0.10.0] <a name="v0.10.0"></a>

### Added <a name="v0.10.0/added"></a>

- Add support for __routing-config__ using existing config-files or a str-parser.
  - Let the new routing-config specify the metrics used by the `Dijkstra`.
  - Let the new routing-config specify preferences for each metric.
  - Let the big Config being built by a ProtoConfig to support dependencies between Config-components (like `cfg.routing`, having a list of ids, depends on the mapping from id to idx, which is stored in `cfg.graph`).
  - Add tests considering the routing-config.
- Support __multiple metrics__ in `Dijkstra`, using dot-product with a preference-vector.
- Little __build-script__ for convenience.
- Create __module `helpers`__ collecting handy functions and adding approximation-comparison for `f32` (like `ApproxEq` or `ApproxCmp`).


### Changed <a name="v0.10.0/changed"></a>

- Rename __container-structs__ in graph to accessors, since they are only accessing, not owning the graph's data.
- Refactor and update __`notes.md`__.
- Rename __metric-categories__ in configs as the expected unit (like `Length` -> `Meters`), because explicit is better than implicit.
- Extend __binary `osmgraphing`__
  - Move complexity from examples to binary.
  - Make __examples__ less complex.
  - Let binary accept __logging-level__ as cli-arg.
- Move __module `defaults`__ from `network` to a global module.
  Rename its content accordingly and extend it.
- Refactor __metrics__
  - __Replace units__ `Meters` by `Kilometers` and `Milliseconds` by `Seconds` to keep numbers nearer to `1.0`, which is nearer to the routing-preferences (`alpha`).
    This, hopefully, improves routing-calculations' accuracy.
  - Let very small __non-zero metrics__, which are around (or exactly) `0.0`, be `std::f32::EPSILON` instead of `1.0`.
  - Remove trait `Metric` for more simplicity.
  - Replace all metrics and calculations in `u32` by __`f32`__.
    Coordinates are included and uses `f32` from now.
  - Add __tests__ for unit-conversions/-calculations.
- Improve __memory-consumption__ by using inlining via `SmallVec` at certain spots.
  - Makes the use of `par_iter()` (instead of `iter()`) in `pbf`-parser great again.
- Simplify __graph-access-functions__ by replacing return-values with `panic!`s.
- Rename all occurences of `type` to `category`.
- Make __test-implementations__ fabulous again.
  - Now, the main-test-files are separated and have a cleaner overview than the underscore-naming-convention and the first approach, where all tests have been in one single test-script.
  - Create a module `helpers` containing general implementations, which can be used by the real test-functions.
    This makes the tests cleaner.
  - Testing configs is part of `parsing`-tests.


### Deprecated <a name="v0.10.0/deprecated"></a>

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- The link to `doc.rs` is hardcoded to `major.minor.patch=0.y.z` because `docs.rs` chooses version `1.0.0` though it's yanked..


### Removed <a name="v0.10.0/removed"></a>

- Remove __`Astar`__ completely, since this project will be used with multiple (custom) metrics and a graph contracted via contraction hierarchies.
  The old implementation is kept in a `kutgw`-branch.
  This makes the routing-module much less complex.
  - Remove __`unidirectional Dijkstra`__ since the `bidirectional Dijkstra` uses only one priority-queue, so overhead for short routes is, if existent, very small.
  - Remove __generics__, since their main-purpose has been supporting estimation-functions of `Astar`.
    For the cost-functions, use the metric-indices from the routing-config instead.
  - Remove some `CostNode`-implementations.
  - Remove `routing::factory`.
- Remove graph-functions __accessing specific metrics__, so only one access-method remains.
- Replace branch `master` by `nightly` to emphasize the difference between `release`s and `master`.


### Fixed <a name="v0.10.0/fixed"></a>

- Fix markdown-references in old headings in __`CHANGELOG.md`__.
- Replace __`std::process::exit(...)`__ by `panic!(...)` to improve feedback, e.g. in tests (where logging-messages are swallowed).


## [v0.9.0][github/self/v0.9.0] <a name="v0.9.0"></a>

### Added <a name="v0.9.0/added"></a>

- Implement `zero()` for `geo::Coordinate`.
- Implement trait `Metric` for `u32`.
- Add __format-check__ to github-action (`cargo fmt -- --check`).
- Support __mulitple metrics__, where the number is only known during runtime.
- __Parse graph__ with config instead of map-file, which can be provided as `yaml`-file.
  - Let __metrics__ have __ids__.
  - Let __routing-algorithms__ access graph-metrics with metric-idx.
  - Add __default-configs__ for maps `isle-of-man` and `simple-stuttgart`.
  - Describe every option in a __full config__ (like a __schema__).
  - Support for __different vehicle-categories__.
  - Add __tests__ for deserializing the default-configs.
- Add module __`helpers`__ for general, handy implementations.
  - The struct `MapFileExt` (name before: `Type`) is being moved from module `parsing` to here.
  - __Initializing logging-levels__ is being moved from examples and executables to here.
  - In the future: dot-product


### Changed <a name="v0.9.0/changed"></a>

- Implement clean __ordering-__ and __equal-traits__ for `CostNode`s in the routing-modules.
- Let __github-action__ upload results of benches in a folder called like the commit-hash.
- Let __github-action__ deploy-and-tag only in a branch called `release` to remove `continue-on-error`
- Improve memory-usage, performance and code-style of __metrics__ and __graphbuilding__.
  - Store metrics in the graph as __`Vec<Vec<u32>>`__ (instead of `Vec<Vec<MetricU32>>` or multiple vecs).
  - Access metrics as `u32` or access it as metric (like `Meters`).
  - Let __graphbuilder__ add metrics with __limited memory-usage__.
  - __Consume metrics after adding__ them to graph, but keep ids.
  - __Sort proto-edges unstable__ to sort them fully in-place.
- Simplify __indices-structs__ and __metric-structs__.
  - Make underlying `u32`-values implicit (__`Struct(u32)`__) instead of explicit (`Struct { value: u32 }`).
  - Implement __`Deref`__ and __`DerefMut`__ for them, replacing `value()`, `to_usize()` and similar.
- Push __proto-edges__ in __graphbuilder__ as struct, not as separate attributes.
- Make parser-functions dependent of `self` to add __`preprocessing`-phase__.
  - The __`fmi`-parser__ uses this `preprocessing`-phase to determine the __node- and edge-ranges__ in the provided file using the counts at the beginning of a `fmi`-map-file.
- Rename structs and fields, whose names based on __"type"__, because this is a reserved keyword.
- Make `pbf`-parser __single-threaded__, since runtime's bottleneck is allocating memory many times.
  The runtime for parsing ways single-threaded and multi-threaded was identical (`3:30 minutes` for multi-threaded, `3:20 minutes` for single-threaded).
  The times are much faster, if the RAM has remaining capacity and doesn't have to use the swap-partition, which has been tested with a Germany-fmi-file with half the number of nodes as the Germany-pbf-file.
- Refactor __tests__ making their names uniformly.


### Deprecated <a name="v0.9.0/deprecated"></a>

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- The link to `doc.rs` is hardcoded to `major.minor.patch=0.y.z` because `docs.rs` chooses version `1.0.0` though it's yanked..
- `CHANGELOG.md` has wrong markdown-references in old headings.


### Removed <a name="v0.9.0/removed"></a>

- Remove `MetricU32`.
- Due to the new configs, __edge-distances can not be calculated__ for some edges, which are missing this value, but only for all or none.


### Fixed <a name="v0.9.0/fixed"></a>

- Improve graphbuilder's memory-usage.
  By estimating the amount of proto-edges referring to `200 MB` (could be changed in the future), the graphbuilder can add only these proto-edges before reallocating.
  This limits the needed memory-usage.
- Add labels in benches to identify non-existent ids/indices.


## [v0.8.0][github/self/v0.8.0] <a name="v0.8.0"></a>

### Added <a name="v0.8.0/added"></a>

- Add wiki-content to cargo-documentation and extend `README.md`
- Add file `notes.md` for information, which
  - is not needed anywhere else.
  - is future-documentation.
  - is a future-issue.
- Stop ignoring `Cargo.lock` in `.gitignore`.
- Add __playground-example__ for generating __random numbers__.
- Extend __routing__
  - Implement a bidirectional Astar.
  - Implement a real Dijkstra without estimation-function, meaning explicitly instead of implicitly having the estimation-function returning zero.
  - Extend __routing-factory__ accordingly.
  - __Benchmarks__ for routing-algorithms, added to github-actions as new job uploading results as __artifacts__.
  - Add __new testing-map with respective tests__ especially designed for __baiting__ bidirectional Astar.
    It has paths between some nodes, whose best path wrt smallest weight is not the best path wrt hop-distance.
  - Add random-route-pairs to __astar-example__ basing on random numbers generated by a seeded uniformly distribution.
- Add trait `Display` as dependency for trait `Metric`.


### Changed <a name="v0.8.0/changed"></a>

- Change graph-building
  - __Reduce additional memory-usage__ when building graph.
    Now, when data is not used anymore, but already stored in the graph, it is dropped.
  - Remove `way-id` from the graphbuilder since it is not used anyways.
  - Let parser already deliver metrics instead of primitives.
- Extract the github-action-step `deploy-and-tag` as separate job.
- Change routing
  - Refactor complete module to improve code-structure of new modules (-> see section [`Added`](#v0.8.0/added)).
  - Make `paths`-module more __public__.
    Still keep access to underlying implementation-struct (`VecPath` or `HashPath`) __private__ to be flexible over changes.
  - Merge some __test__-modules to __reduce folder-complexity and redundant code__.
- Rename some methods more explicetly, like `geo::Coordinate::from -> geo::Coordinate::from_64`.


### Deprecated <a name="v0.8.0/deprecated"></a>

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- The link to `doc.rs` is hardcoded to `major.minor.patch=0.y.z` because `docs.rs` chooses version `1.0.0` though it's yanked..
- `CostNode`s in the routing-modules are implementing ordering- and equal-traits sloppy.


### Fixed <a name="v0.8.0/fixed"></a>

- In `CHANGELOG.md`, markdown-links (not URLs!) in the table-of-contents should be persistent, meaning `#v1.0.0-yanked` should be `#v1.0.0`.
- In example `astar`, the distance has been printed with two units (`123 m m`).
- Add an alternative best route to a test-case of the small graph, that has been missing.


## [v0.7.1][github/self/v0.7.1] <a name="v0.7.1"></a>

### Changed

- Fix link to `docs.rs` in `README.md`


### Deprecated

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- Routing should be extended by a `bidirectional Astar`.
- The link to `doc.rs` is hardcoded to `major.minor.patch=0.y.z` because `docs.rs` chooses version `1.0.0` though it's yanked..
- Building the graph uses too much additional memory due to not dropping unused data though it is already stored in the graph.


## [v0.7.0][github/self/v0.7.0] <a name="v0.7.0"></a>

### Added <a name="v0.7.0/added"></a>

- Implement __access to forward-edges and backward-edges__, as preparation to the `bidirectional Dijkstra`, .
  - Process queries for forward-edges and backward-edges by the same code, due to the new pattern with the __shallow containers__.
    To achieve this without additional performance-cost, use a index-mapping for offsets, while accessing all other components (node-indices and metrics) directly.
  - Extend graph-construction-tests for backward-edges.
- Add __documentation__ for the graph.
- Add a __metric-system__ replacing primitive data-types.
  - Support typical calculations as `v = s/t`, typed correctly (`meters / milliseconds -> km/h`).


### Changed <a name="v0.7.0/changed"></a>

- Comment in `Cargo.toml`, over changing the version, to not forget changing the `CHANGELOG.md`.
- Refactor the graph by a new pattern.
  - Add new examples playing around with different patterns (`RefCell` vs `moving` vs `borrowing`).
  - Store the data in arrays in one single struct (the graph), while granting access over layer-structs borrowing these arrays and executing user-queries.
    This makes maintainability without breaking changes easier.
  - Let the graph-interface allow access via `NodeContainer` and `EdgeContainer`.
- Replace the use of usize by new structs, `NodeIdx` and `EdgeIdx`, to control the access to graph-components.
- Refactor logging slightly by adding progress-bars to parsing and building.
- Use a new type-parameter in the `Astar` for a metric-type, which are added in this release.
  - Change the access to best-path-algorithms slightly.


### Deprecated <a name="v0.7.0/deprecated"></a>

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- Routing should be extended by a `bidirectional Astar`.


### Removed <a name="v0.7.0/removed"></a>

- The feature showing whether an `edge is enabled` is being removed to make handling backward-edges easier.
- `Edge-ID`s are not needed and hence removed.
- When building and finalizing the graph, `ProtoNode`s and `ProtoEdge`s don't need ordering implemented, hence these implementations are removed.


## [v1.0.0-yanked][github/self/v1.0.0-yanked] <a name="v1.0.0"></a>

### Added <a name="v1.0.0/added"></a>

- Add `CHANGELOG.md`. `:)`
- Add GitHub-action automatically testing and deploying.
  As improvement over travis-ci, it tags commits automatically if the `Cargo.toml`-version has changed and pushed.
  Before, both (tag and `Cargo.toml`) had to be updated.


### Changed <a name="v1.0.0/changed"></a>

- The `README.md` has no longer `News` due to the new `CHANGELOG.md`.


### Deprecated <a name="v1.0.0/deprecated></a>

- `CHANGELOG.md` contains empty version-descriptions.
- Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
- Inconsistent `semver` in tagging -> probably `cargo yank VERSION` needed
- The `graph`-interface should allow access via `node`- and `edge`-containers
  - This would improve maintainability and reduce number of breaking changes in the future.
  - Further, it would allow using multidimensional metrics easier an probably improves caching thanks to `Structure of Arrays` instead of currently used `Array of Structures`
- The `graph` containing forward-edges will be extended by backward-edges.
  - The feature showing whether an `edge is enabled` will be removed.
- Routing should be extended by a `bidirectional Astar`.


### Removed <a name="v1.0.0/removed></a>

- The `braess-optimization` has been removed (to a `kutgw`-branch), since it's just kind of a big playground and interferes with future code, at least when testing.
- `Travis-CI` has been replaced by `GitHub`-workflows (-> see section [`Added`](#v1.0.0/added)).


## [v0.6.1][github/self/v0.6.1] <a name="v0.6.1"></a>

### Deprecated <a name="v0.6.1/deprecated"></a>

- todo


## [v0.6.0][github/self/v0.6.0] <a name="v0.6.0"></a>

### Deprecated <a name="v0.6.0/deprecated"></a>

- todo


## [v0.5.0][github/self/v0.5.0] <a name="v0.5.0"></a>

### Deprecated <a name="v0.5.0/deprecated"></a>

- todo


## [v0.4.1][github/self/v0.4.1] <a name="v0.4.1"></a>

### Deprecated <a name="v0.4.1/deprecated"></a>

- todo


## [v0.4.0][github/self/v0.4.0] <a name="v0.4.0"></a>

### Deprecated <a name="v0.4.0/deprecated"></a>

- todo


## [v0.3.1][github/self/v0.3.1] <a name="v0.3.1"></a>

### Deprecated <a name="v0.3.1/deprecated"></a>

- todo


## [v0.3.0][github/self/v0.3.0] <a name="v0.3.0"></a>

### Deprecated <a name="v0.3.0/deprecated"></a>

- todo


## [v0.2.4][github/self/v0.2.4] <a name="v0.2.4"></a>

### Deprecated <a name="v0.2.4/deprecated"></a>

- todo


## [v0.2.3][github/self/v0.2.3] <a name="v0.2.3"></a>

### Deprecated <a name="v0.2.3/deprecated"></a>

- todo


## [v0.2.2][github/self/v0.2.2] <a name="v0.2.2"></a>

### Deprecated <a name="v0.2.2/deprecated"></a>

- todo


## [v0.2.1][github/self/v0.2.1] <a name="v0.2.1"></a>

### Deprecated <a name="v0.2.1/deprecated"></a>

- todo


## [v0.2.0][github/self/v0.2.0] <a name="v0.2.0"></a>

### Deprecated <a name="v0.2.0/deprecated"></a>

- todo


## [v0.1.5][github/self/v0.1.5] <a name="v0.1.5"></a>

### Deprecated <a name="v0.1.5/deprecated"></a>

- todo


## [v0.1.4][github/self/v0.1.4] <a name="v0.1.4"></a>

### Deprecated <a name="v0.1.4/deprecated"></a>

- todo


## [v0.1.3][github/self/v0.1.3] <a name="v0.1.3"></a>

### Deprecated <a name="v0.1.3/deprecated"></a>

- todo


## [v0.1.2][github/self/v0.1.2] <a name="v0.1.2"></a>

### Deprecated <a name="v0.1.2/deprecated"></a>

- todo


## [v0.1.1][github/self/v0.1.1] <a name="v0.1.1"></a>

### Deprecated <a name="v0.1.1/deprecated"></a>

- todo


## [v0.1.0][github/self/v0.1.0] <a name="v0.1.0"></a>

### Deprecated <a name="v0.1.0/deprecated"></a>

- todo


[keepachangelog]: https://keepachangelog.com/en/
[semver]: https://semver.org/

[github/self/unreleased]: https://github.com/dominicparga/osmgraphing/compare/v0.11.0...HEAD
[github/self/v0.11.0]: https://github.com/dominicparga/osmgraphing/compare/v0.10.0...v0.11.0
[github/self/v0.10.0]: https://github.com/dominicparga/osmgraphing/compare/v0.9.0...v0.10.0
[github/self/v0.9.0]: https://github.com/dominicparga/osmgraphing/compare/v0.8.0...v0.9.0
[github/self/v0.8.0]: https://github.com/dominicparga/osmgraphing/compare/v0.7.1...v0.8.0
[github/self/v0.7.1]: https://github.com/dominicparga/osmgraphing/compare/v0.7.0...v0.7.1
[github/self/v0.7.0]: https://github.com/dominicparga/osmgraphing/compare/v1.0.0-yanked...v0.7.0
[github/self/v1.0.0-yanked]: https://github.com/dominicparga/osmgraphing/compare/v0.6.1...v1.0.0-yanked
[github/self/v0.6.1]: https://github.com/dominicparga/osmgraphing/compare/v0.6.0...v0.6.1
[github/self/v0.6.0]: https://github.com/dominicparga/osmgraphing/compare/v0.5.0...v0.6.0
[github/self/v0.5.0]: https://github.com/dominicparga/osmgraphing/compare/v0.4.1...v0.5.0
[github/self/v0.4.1]: https://github.com/dominicparga/osmgraphing/compare/v0.4.0...v0.4.1
[github/self/v0.4.0]: https://github.com/dominicparga/osmgraphing/compare/v0.3.1...v0.4.0
[github/self/v0.3.1]: https://github.com/dominicparga/osmgraphing/compare/v0.3.0...v0.3.1
[github/self/v0.3.0]: https://github.com/dominicparga/osmgraphing/compare/v0.2.4...v0.3.0
[github/self/v0.2.4]: https://github.com/dominicparga/osmgraphing/compare/v0.2.3...v0.2.4
[github/self/v0.2.3]: https://github.com/dominicparga/osmgraphing/compare/v0.2.2...v0.2.3
[github/self/v0.2.2]: https://github.com/dominicparga/osmgraphing/compare/v0.2.1...v0.2.2
[github/self/v0.2.1]: https://github.com/dominicparga/osmgraphing/compare/v0.2.0...v0.2.1
[github/self/v0.2.0]: https://github.com/dominicparga/osmgraphing/compare/v0.1.5...v0.2.0
[github/self/v0.1.5]: https://github.com/dominicparga/osmgraphing/compare/v0.1.4...v0.1.5
[github/self/v0.1.4]: https://github.com/dominicparga/osmgraphing/compare/v0.1.3...v0.1.4
[github/self/v0.1.3]: https://github.com/dominicparga/osmgraphing/compare/v0.1.2...v0.1.3
[github/self/v0.1.2]: https://github.com/dominicparga/osmgraphing/compare/v0.1.1...v0.1.2
[github/self/v0.1.1]: https://github.com/dominicparga/osmgraphing/compare/v0.1.0...v0.1.1
[github/self/v0.1.0]: https://github.com/dominicparga/osmgraphing/releases/tag/v0.1.0
