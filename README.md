# osmgraphing

[![Build Status nightly][github/self/actions/badge]][github/self/actions]

[![Tag][github/self/tags/badge]][github/self/tags]
[![Crates.io][crates.io/self/badge]][crates.io/self]
[![Docs][docs.rs/self/badge]][docs.rs/self]

[![Changelog][github/self/blob/changelog/badge]][github/self/blob/changelog]
[![Last commit][github/self/last-commit/badge]][github/self/last-commit]

[![License][github/self/license/badge]][github/self/license]

Welcome to the `osmgraphing`-repo! `:)`
Goal of this repo is parsing [openstreetmap][osm]-data to calculate traffic-routes and different related use-cases on it.
This repo deals with analyzing selfish routing and learning metrics for balancing load in street-networks.
All calculations should be done effectively on a single desktop instead of an expensive cluster.


## Setup and usage

`cargo` is the build-tool of Rust and can be used to run everything except scripts in `scripts/`.
`cargo run` will give you help, e.g. it tells you to use `cargo run --example`.
Running this command will print names of runnable examples.
Further, refer to the [examples][github/self/tree/examples] for more details, or to [cargo-docs][docs.rs/self] to get details about the repo's setup and implementation.

Downloaded osm-data is provided in xml (`osm`) or binary (`pbf`), where nodes are related to location in latitude and longitude.
Problems will be the size-limit when downloading from [openstreetmap][osm], but there are other osm data providers like [geofabrik][geofabrik] for instance.

For testing, some simple text-based format `fmi` is used.
Since they are created manually for certain tasks, parsing them - generally speaking - is unstable.
Tools creating `fmi`-files are [pbfextractor][github/lesstat/pbfextractor] and [multi-ch-constructor][github/lesstat/multi-ch-constructor] (working with contraction-hierarchies).


## Requirements for large maps

In general, the requirements depend on the size of the parsed map and your machine.
Following numbers base on an __8-core-CPU__ and the `pbf`-map `Germany` running on `archlinux`.
Further, they base on the assumption, that you don't use more than 5 metrics (besides ignore and ids), because up to 5 metrics are inlined with `SmallVec`.
You should change the number of inlined metrics according to your needs in the module `defaults`.

- Parsing `Germany` (~50 million nodes, ~103 million edges) needs around __10 GB of RAM__.
  After parsing, the memory-needs are less (up to few `GB`s) due to the optimized graph-structure.
- Preprocessing `Germany` (including parsing) needs around __4 minutes__.
  This highly depends on the number of cores.
- A __routing query__ on `Germany` of length `620 km` takes around __16 seconds__ with `bidirectional Dijkstra`.
  This could be improved by removing intermediate nodes (like `b` in `a->b->c`), but they are kept for now.
  An `Astar` is not used anymore, because its only purpose is reducing the search-space, which can be reduced much more using `Contraction Hierarchies`.
  Further, `Astar` has issues when it comes to multiple or custom metrics, because of the metrics' heuristics.

Small maps like `Isle of Man` run on every machine and are parsed in less than a second.


## Credits

The project started in the mid of 2019 as a student project.
This page honors the workers and helpers of this project, sorted by their last names.

__[Florian Barth][github/lesstat]__  
is the supervisor of the project since beginning and is always helping immediately with his experience and advice.

__[Dominic Parga Cacheiro][github/dominicparga]__  
has been part of the project's first weeks when project-planning and learning Rust was on the scope.
He continues the work and is writing and improving the simulation.

__[Jena Satkunarajan][github/jenasat]__  
has been part of the project's first weeks when project-planning and learning Rust was on the scope.
He has implemented the first (and running) approach of the `A*`-algorithm.


[crates.io/self]: https://crates.io/crates/osmgraphing
[crates.io/self/badge]: https://img.shields.io/crates/v/osmgraphing?style=for-the-badge
[docs.rs/self]: https://docs.rs/osmgraphing/0/
[docs.rs/self/badge]: https://img.shields.io/crates/v/osmgraphing?color=informational&label=docs&style=for-the-badge
[geofabrik]: https://geofabrik.de
[github/dominicparga]: https://github.com/dominicparga
[github/jenasat]: https://github.com/JenaSat
[github/lesstat]: https://github.com/lesstat
[github/lesstat/multi-ch-constructor]: https://github.com/Lesstat/multi-ch-constructor
[github/lesstat/pbfextractor]: https://github.com/Lesstat/pbfextractor
[github/self/actions]: https://github.com/dominicparga/osmgraphing/actions
[github/self/actions/badge]: https://img.shields.io/github/workflow/status/dominicparga/osmgraphing/Rust?label=nightly-build&style=for-the-badge
[github/self/blob/changelog]: https://github.com/dominicparga/osmgraphing/blob/nightly/CHANGELOG.md
[github/self/blob/changelog/badge]: https://img.shields.io/badge/CHANGELOG-nightly-blueviolet?style=for-the-badge
[github/self/last-commit]: https://github.com/dominicparga/osmgraphing/commits
[github/self/last-commit/badge]: https://img.shields.io/github/last-commit/dominicparga/osmgraphing?style=for-the-badge
[github/self/license]: https://github.com/dominicparga/osmgraphing/blob/nightly/LICENSE
[github/self/license/badge]: https://img.shields.io/github/license/dominicparga/osmgraphing?style=for-the-badge
[github/self/tags]: https://github.com/dominicparga/osmgraphing/tags
[github/self/tags/badge]: https://img.shields.io/github/v/tag/dominicparga/osmgraphing?sort=semver&style=for-the-badge
[github/self/tree/examples]: https://github.com/dominicparga/osmgraphing/tree/nightly/examples
[github/self/wiki/usage]: https://github.com/dominicparga/osmgraphing/wiki/Usage
[osm]: https://openstreetmap.org
