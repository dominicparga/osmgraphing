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
This repo will be involved in dealing with the analysis of selfish routing and learning metrics for balancing load in street-networks.
However, if a self-written parser-module does exist, every map-format supported by this module (e.g. own `csv`-like formats) can be used, which doesn't need to be a street-network.

All calculations will be optimized for a single desktop instead of a more expensive cluster.


## Reason for `version < 1.0.0` <a name="version"></a>

I'm currently building this library for my master-thesis (submission `August 2020`), leading to interface-changes with breaking changes (at least) every few weeks, why version `1.0.0` is not supported yet.
However, the underlying parser and graph-structure are working very stable, efficiently, tested with different maps (see `resources/`), and will be used to simulate different routing-scenarios, so version `1.0.0` should be reached soon. `:)`


## Copyright and License <a name="copyright_and_license"></a>

Please refer to `LICENSE.md`.


## Table of contents <a name="toc"></a>

1. [Reason for version < 1.0.0][self/version]
1. [Copyright and License][self/copyright_and_license]
1. [Table of contents][self/toc]
1. [Setup and usage][self/setup-and-usage]
    1. [Long story short][self/long-story-short]
    1. [Downloading and generating maps][self/downloading-and-generating]
    1. [Editing the config][self/editing-the-config]
    1. [Inlined metrics][self/inlined-metrics]
    1. [Requirements for large maps (e.g. countries)][self/large-maps]
    1. [Contraction-Hierarchies][self/contraction-hierarchies]
1. [Balancing][self/balancing]
1. [Credits][self/credits]


## Setup and usage <a name="setup-and-usage"></a>

### Long story short <a name="long-story-short"></a>

Rust has a build-tool called `cargo`, which can be used to run everything except scripts in `scripts/`.

```zsh
# Build the binary for parsing maps and do routing
cargo build --release
# Parse isle-of-man
./target/release/osmgraphing --config resources/isle-of-man_2020-03-14/osm.pbf.yaml
# Further execution-info
./target/release/osmgraphing --help
```

Above binary will throw an error, since you probably haven't downloaded the map-file mentioned in the config.
You can download `pbf`-files from [geofabrik][geofabrik].
When editing the config, take [`resources/blueprint.yaml`][github/self/blob/blueprint.yaml] as guide.

For using the balancer, you have to enable features licensed under the `GPL-3.0`.

```zsh
# Update git-submodules used in the balancer
git submodule update --init --recursive
# Also build features licensed under the `GPL-3.0`.
cargo build --release --features 'gpl-3.0'
./target/release/balancer --help
```

You can find a detailled config-blueprint in `resources/` and a balancer-example in `resources/isle_of_man/`.
The results can be visualized with the python-module in `scripts/`.

### Downloading and generating maps <a name="downloading-and-generating"></a>

Downloaded osm-data is provided in xml (`osm`) or binary (`pbf`), where nodes are related to location in latitude and longitude.
Problems will be the size-limit when downloading from [openstreetmap][osm], but there are other osm data providers like [geofabrik][geofabrik] for instance.

For testing, some simple text-based format `fmi` is used.
Since they are created manually for certain tasks, parsing them - generally speaking - is unstable.
However, this repository has a generator, which can create such `fmi`-files from `pbf`- or other `fmi`-files (e.g. for different metric-order).
A tool for creating `fmi`-map-files, containing graphs contracted via contraction-hierarchies, is [multi-ch-constructor][github/lesstat/multi-ch-constructor], which is used in the balancer.


### Editing the config <a name="editing-the-config"></a>

Every option of a config is described in [`resources/blueprint.yaml`][github/self/blob/blueprint.yaml].
The binaries `osmgraphing` and `balancer` (binaries are in `target/release` after release-building) use the config for different use-cases.

### Inlined metrics <a name="inlined-metrics"></a>

Metrics are inlined using [`SmallVec`][github/servo/rust-smallvec].
This improves performance and saves several GB.
If your config uses less metrics than you have compiled to, you will receive a warning.
Further, if the compiled number of inlined metrics is less than the number of your config's metrics, the graph can't be parsed and you receive an error.
In this case, you must change the number of inlined metrics according to your needs.
You can find this number in the module [`defaults`][github/self/blob/defaults.rs] as `SMALL_VEC_INLINE_SIZE` (`May 19th, 2020`).


### Requirements for large maps (e.g. countries) <a name="large-maps"></a>

In general, the requirements depend on the size of the parsed map (also same map of different dates) and your machine.
Following numbers base on an __8-core-CPU__ and the `pbf`-maps from `March 14th, 2020` running on `archlinux` with __16 GB RAM__.
Basing on the numbers below, without doing further detailled benchmarks, the memory-usage scales linearly with the graph-size with a growth-factor of `0.5`.
Hence you could expect around `2x` `RAM`-usage for `4x` graph-size (meaning `4x` node- and `4x` edge-count).

- Parsing `Germany.pbf` (4 metrics, ~51 million nodes, ~106 million edges) needs around __14 GB of RAM__ at peak.
  After parsing, the memory-needs are much lower due to the optimized graph-structure.
- Preprocessing `Germany.pbf` (including parsing) needs less than __4 minutes__.
- A __routing query__ on `Germany.pbf` of distance around `600 km` takes around __22 seconds__ with `bidirectional Dijkstra`, highly depending on the specific src-dst-pair (and its search-space).
  This could be improved by removing intermediate nodes (like `b` in `a->b->c`), but they are kept for now.
  Maybe, they are needed for precise/realistic traffic-simulation.
  An `Astar` is not used anymore, because its only purpose is reducing the search-space, which can be reduced much more using [`Contraction Hierarchies`][self/contraction-hierarchies].
  Further, `Astar` has issues when it comes to multiple or custom metrics, because of the metrics' heuristics.

Small maps like `Isle_of_Man.pbf` (~50_000 nodes, ~107_000 edges) run on every machine and are parsed in less than a second.

The German state `Baden-Württemberg.pbf` (~9 million nodes, ~18 million edges) needs less than __5 GB RAM__ at peak and around __30 seconds__ to parse.


### Contraction-Hierarchies <a name="contraction-hierarchies"></a>

For speedup, this repository supports graphs contracted via contraction-hierarchies.
The repository [`lesstat/multi-ch-constructor`][github/lesstat/multi-ch-constructor] generates contracted graphs from `fmi`-files of a certain format (see below).
This repository, `osmgraphing`, uses the fork `dominicparga/multi-ch-constructor` as submodule for its ch-graphs.
For reproducability, the used steps are listed below.

First of all, the tool `multi-ch` needs an `fmi`-map-file of specific format as input.
To generate such a `fmi`-map-file in the correct format, the binary `osmgraphing` can be used with a config following the [defined requirements][github/lesstat/cyclops/blob/README].
See `resources/blueprint.yaml` for detailled infos about configs.

The `ignored`s and placeholders (e.g. `ch-level`) in the config are important, because the `multi-ch-constructor` needs them.
Besides that, the `multi-ch-constructor` uses node-indices as ids, leading to errors when the mapping `node -> indices [0; n]` is not surjective.
Therefore, export the graph's edges using `src-idx` and `dst-idx` instead of `srd-id` and `dst-id`.

The `multi-ch`-tool needs 3 counts at the file-beginning: metric-count (dimension), node-count, edge-count.
The `osmgraphing`-binary does add these counts in this order.

Before the `multi-ch`-tool can be used, it has to be built.
For the sake of optimization, you have to set the metric-count as dimension in [multi-ch-constructor/src/multi_lib/graph.hpp, line 49][github/lesstat/multi-ch-constructor/change-dim].
Set this dimension according to the dimension in the previously generated `fmi`-file.
The fork allows this via `cmake`.
See its README for more info.

> Note that the multi-ch-constructor is not deterministic (March 12th, 2020).
> Using it does only speedup your queries, but due to a different resulting order in the priority, or rounding-errors, it could lead to different paths of same weight.


## Balancing <a name="balancing"></a>

See `./target/balancer --help`.


## Credits <a name="credits"></a>

The project started in the mid of 2019 as a student project.
This section honors the workers and helpers of this project, sorted by their last names.

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
[github/lesstat/cyclops/blob/README]: https://github.com/Lesstat/cyclops/blob/master/README.md#graph-data
[github/lesstat/multi-ch-constructor]: https://github.com/Lesstat/multi-ch-constructor
[github/lesstat/multi-ch-constructor/change-dim]: https://github.com/Lesstat/multi-ch-constructor/blob/bec548c1a1ebeae7ac19d3250d5473199336d6fe/src/multi_lib/graph.hpp#L49
[github/self/actions]: https://github.com/dominicparga/osmgraphing/actions
[github/self/actions/badge]: https://img.shields.io/github/workflow/status/dominicparga/osmgraphing/Rust?label=nightly-build&style=for-the-badge
[github/self/blob/blueprint.yaml]: https://github.com/dominicparga/osmgraphing/blob/nightly/resources/blueprint.yaml
[github/self/blob/changelog]: https://github.com/dominicparga/osmgraphing/blob/nightly/CHANGELOG.md
[github/self/blob/changelog/badge]: https://img.shields.io/badge/CHANGELOG-nightly-blueviolet?style=for-the-badge
[github/self/blob/defaults.rs]: https://github.com/dominicparga/osmgraphing/blob/nightly/src/defaults.rs
[github/self/last-commit]: https://github.com/dominicparga/osmgraphing/commits
[github/self/last-commit/badge]: https://img.shields.io/github/last-commit/dominicparga/osmgraphing?style=for-the-badge
[github/self/license]: https://github.com/dominicparga/osmgraphing/blob/nightly/LICENSE.md
[github/self/license/badge]: https://img.shields.io/badge/LICENSE-Apache--2.0%20OR%20GPL--3.0-green?style=for-the-badge
[github/self/tags]: https://github.com/dominicparga/osmgraphing/tags
[github/self/tags/badge]: https://img.shields.io/github/v/tag/dominicparga/osmgraphing?sort=semver&style=for-the-badge
[github/self/tree/examples]: https://github.com/dominicparga/osmgraphing/tree/nightly/examples
[github/self/wiki/usage]: https://github.com/dominicparga/osmgraphing/wiki/Usage
[github/servo/rust-smallvec]: https://github.com/servo/rust-smallvec
[osm]: https://openstreetmap.org
[self/balancing]: #balancing
[self/contraction-hierarchies]: #contraction-hierarchies
[self/copyright_and_license]: #copyright_and_license
[self/credits]: #credits
[self/downloading-and-generating]: #downloading-and-generating
[self/editing-the-config]: #editing-the-config
[self/inlined-metrics]: #inlined-metrics
[self/large-maps]: #large-maps
[self/long-story-short]: #long-story-short
[self/setup-and-usage]: #setup-and-usage
[self/toc]: #toc
[self/version]: #version
