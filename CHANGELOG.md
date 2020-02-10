# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this project adheres to [Semantic Versioning][semver].


## Table of contents

1. [Unreleased](#unreleased)
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

### Added

\-


### Changed

- Reduce additional memory-usage when building graph.
  When data is not used anymore, but already stored in the graph, it is dropped.
- Remove `way-id` from the graphbuilder since it is not used anyways.


### Deprecated

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- Routing should be extended by a `bidirectional A*`.
- The link to `doc.rs` is hardcoded to `major.minor.patch=0.y.z` because `docs.rs` chooses version `1.0.0` though it's yanked..


### Removed

\-


### Fixed

\-


### Security

\-


## [v0.7.1][github/self/v0.7.1] <a name="v0.7.1"></a>

### Changed

- Fix link to `docs.rs` in `README.md`


### Deprecated

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- Routing should be extended by a `bidirectional A*`.
- The link to `doc.rs` is hardcoded to `major.minor.patch=0.y.z` because `docs.rs` chooses version `1.0.0` though it's yanked..
- Building the graph uses too much additional memory due to not dropping unused data though it is already stored in the graph.


## [v0.7.0][github/self/v0.7.0] <a name="v0.7.0"></a>

### Added

- Implement access to forward-edges and backward-edges, as preparation to the `bidirectional Dijkstra`, .
  - Process queries for forward-edges and backward-edges by the same code, due to the new pattern with the shallow containers.
    To achieve this without additional performance-cost, use a index-mapping for offsets, while accessing all other components (node-indices and metrics) directly.
  - Extend graph-construction-tests for backward-edges.
- Add documentation for the graph.
- Add a metric-system replacing primitive data-types.
  - Support typical calculations as `v = s/t`, typed correctly (`meters / milliseconds -> km/h`).


### Changed

- Comment in `Cargo.toml`, over changing the version, to not forget changing the `CHANGELOG.md`.
- Refactor the graph by a new pattern.
  - Add new examples playing around with different patterns (`RefCell` vs `moving` vs `borrowing`).
  - Store the data in arrays in one single struct (the graph), while granting access over layer-structs borrowing these arrays and executing user-queries.
    This makes maintainability without breaking changes easier.
  - Let the graph-interface allow access via `NodeContainer` and `EdgeContainer`.
- Replace the use of usize by new structs, `NodeIdx` and `EdgeIdx`, to control the access to graph-components.
- Refactor logging slightly by adding progress-bars to parsing and building.
- Use a new type-parameter in the `A*` for a metric-type, which are added in this release.
  - Change the access to best-path-algorithms slightly.


### Deprecated

- Documentation is missing, though comments are very well.
- `CHANGELOG.md` contains empty version-descriptions.
  - Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
  - Inconsistent `semver` in old tags -> probably `cargo yank VERSION` needed
- Routing should be extended by a `bidirectional A*`.


### Removed

- The feature showing whether an `edge is enabled` is being removed to make handling backward-edges easier.
- `Edge-ID`s are not needed and hence removed.
- When building and finalizing the graph, `ProtoNode`s and `ProtoEdge`s don't need ordering implemented, hence these implementations are removed.


## [v1.0.0-yanked][github/self/v1.0.0-yanked] <a name="v1.0.0"></a>

### Added <a name="v1.0.0/added"></a>

- Add `CHANGELOG.md`. `:)`
- Add GitHub-action automatically testing and deploying.
  As improvement over travis-ci, it tags commits automatically if the `Cargo.toml`-version has changed and pushed.
  Before, both (tag and `Cargo.toml`) had to be updated.


### Changed

- The `README.md` has no longer `News` due to the new `CHANGELOG.md`.


### Deprecated

- `CHANGELOG.md` contains empty version-descriptions.
- Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
- Inconsistent `semver` in tagging -> probably `cargo yank VERSION` needed
- The `graph`-interface should allow access via `node`- and `edge`-containers
  - This would improve maintainability and reduce number of breaking changes in the future.
  - Further, it would allow using multidimensional metrics easier an probably improves caching thanks to `Structure of Arrays` instead of currently used `Array of Structures`
- The `graph` containing forward-edges will be extended by backward-edges.
  - The feature showing whether an `edge is enabled` will be removed.
- Routing should be extended by a `bidirectional A*`.


### Removed

- The `braess-optimization` has been removed (to a `kutgw`-branch), since it's just kind of a big playground and interferes with future code, at least when testing.
- `Travis-CI` has been replaced by `GitHub`-workflows (-> see section [`Added`](#v1.0.0/added)).


## [v0.6.1][github/self/v0.6.1] <a name="v0.6.1"></a>

### Deprecated

- todo


## [v0.6.0][github/self/v0.6.0] <a name="v0.6.0"></a>

### Deprecated

- todo


## [v0.5.0][github/self/v0.5.0] <a name="v0.5.0"></a>

### Deprecated

- todo


## [v0.4.1][github/self/v0.4.1] <a name="v0.4.1"></a>

### Deprecated

- todo


## [v0.4.0][github/self/v0.4.0] <a name="v0.4.0"></a>

### Deprecated

- todo


## [v0.3.1][github/self/v0.3.1] <a name="v0.3.1"></a>

### Deprecated

- todo


## [v0.3.0][github/self/v0.3.0] <a name="v0.3.0"></a>

### Deprecated

- todo


## [v0.2.4][github/self/v0.2.4] <a name="v0.2.4"></a>

### Deprecated

- todo


## [v0.2.3][github/self/v0.2.3] <a name="v0.2.3"></a>

### Deprecated

- todo


## [v0.2.2][github/self/v0.2.2] <a name="v0.2.2"></a>

### Deprecated

- todo


## [v0.2.1][github/self/v0.2.1] <a name="v0.2.1"></a>

### Deprecated

- todo


## [v0.2.0][github/self/v0.2.0] <a name="v0.2.0"></a>

### Deprecated

- todo


## [v0.1.5][github/self/v0.1.5] <a name="v0.1.5"></a>

### Deprecated

- todo


## [v0.1.4][github/self/v0.1.4] <a name="v0.1.4"></a>

### Deprecated

- todo


## [v0.1.3][github/self/v0.1.3] <a name="v0.1.3"></a>

### Deprecated

- todo


## [v0.1.2][github/self/v0.1.2] <a name="v0.1.2"></a>

### Deprecated

- todo


## [v0.1.1][github/self/v0.1.1] <a name="v0.1.1"></a>

### Deprecated

- todo


## [v0.1.0][github/self/v0.1.0] <a name="v0.1.0"></a>

### Deprecated

- todo


[keepachangelog]: https://keepachangelog.com/en/
[semver]: https://semver.org/

[github/self/unreleased]: https://github.com/dominicparga/osmgraphing/compare/v0.7.1...HEAD
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
