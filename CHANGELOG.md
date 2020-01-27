# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this project adheres to [Semantic Versioning][semver].


## Table of contents

1. [Unreleased](#unreleased)
1. [v1.0.0](#v1.0.0)
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


## [Unreleased] <a name="unreleased"></a>

### Added <a name="unreleased/added"></a>

- Comment in `Cargo.toml`, over changing the version, to not forget changing the `CHANGELOG.md`.


### Changed

\-


### Deprecated

- `CHANGELOG.md` contains empty version-descriptions.
- Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
- Inconsistent `semver` in tagging -> probably `cargo yank VERSION` needed
- The `graph`-interface should allow access via `node`- and `edge`-containers for better maintainability without breaking changes.
- The `graph` containing forward-edges will be extended by backward-edges.
- Routing should be extended by a `bidirectional A*`.


### Removed

- The feature showing whether an `edge is enabled` is being removed to make handling backward-edges easier.


### Fixed

\-


### Security

\-


## [v1.0.0][github/self/v1.0.0] <a name="v1.0.0"></a>

### Added <a name="v1.0.0/added"></a>

- Add `CHANGELOG.md`. `:)`
- Add `GitHub`-workflow automatically testing and deploying.
  As improvement over `travis-ci`, it tags commits automatically if the `Cargo.toml`-version has changed.


### Changed

- The `README.md` has no longer `News` due to the new `CHANGELOG.md`.


### Deprecated

- `CHANGELOG.md` contains empty version-descriptions.
- Replace existing tags with ones referring to `CHANGELOG.md` and add old tag-texts to the `CHANGELOG.md`
- Inconsistent `semver` in tagging -> probably `cargo yank VERSION` needed
- The `graph`-interface should allow access via `node`- and `edge`-containers for better maintainability without breaking changes.
- The `graph` containing forward-edges will be extended by backward-edges.
  - The feature showing whether an `edge is enabled` will be removed.
- Routing should be extended by a `bidirectional A*`.


### Removed

- The `braess-optimization` has been removed (to a `kutgw`-branch), since it's just kind of a big playground and interferes with future code, at least when testing.
- `Travis-CI` has been replaced by `GitHub`-workflows (-> see section [`Added`](#v1.0.0/added)).


### [v0.6.1][github/self/v0.6.1] <a name="v0.6.1"></a>

#### Deprecated

- todo


### [v0.6.0][github/self/v0.6.0] <a name="v0.6.0"></a>

#### Deprecated

- todo


### [v0.5.0][github/self/v0.5.0] <a name="v0.5.0"></a>

#### Deprecated

- todo


### [v0.4.1][github/self/v0.4.1] <a name="v0.4.1"></a>

#### Deprecated

- todo


### [v0.4.0][github/self/v0.4.0] <a name="v0.4.0"></a>

#### Deprecated

- todo


### [v0.3.1][github/self/v0.3.1] <a name="v0.3.1"></a>

#### Deprecated

- todo


### [v0.3.0][github/self/v0.3.0] <a name="v0.3.0"></a>

#### Deprecated

- todo


### [v0.2.4][github/self/v0.2.4] <a name="v0.2.4"></a>

#### Deprecated

- todo


### [v0.2.3][github/self/v0.2.3] <a name="v0.2.3"></a>

#### Deprecated

- todo


### [v0.2.2][github/self/v0.2.2] <a name="v0.2.2"></a>

#### Deprecated

- todo


### [v0.2.1][github/self/v0.2.1] <a name="v0.2.1"></a>

#### Deprecated

- todo


### [v0.2.0][github/self/v0.2.0] <a name="v0.2.0"></a>

#### Deprecated

- todo


### [v0.1.5][github/self/v0.1.5] <a name="v0.1.5"></a>

#### Deprecated

- todo


### [v0.1.4][github/self/v0.1.4] <a name="v0.1.4"></a>

#### Deprecated

- todo


### [v0.1.3][github/self/v0.1.3] <a name="v0.1.3"></a>

#### Deprecated

- todo


### [v0.1.2][github/self/v0.1.2] <a name="v0.1.2"></a>

#### Deprecated

- todo


### [v0.1.1][github/self/v0.1.1] <a name="v0.1.1"></a>

#### Deprecated

- todo


### [v0.1.0][github/self/v0.1.0] <a name="v0.1.0"></a>

#### Deprecated

- todo




[keepachangelog]: https://keepachangelog.com/en/
[semver]: https://semver.org/

[Unreleased]: https://github.com/dominicparga/osmgraphing/compare/v1.0.0...HEAD
[github/self/v1.0.0]: https://github.com/dominicparga/osmgraphing/compare/v0.6.1...v1.0.0
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
