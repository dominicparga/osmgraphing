# osmgraphing

[![Build Status latest][travis/latest/badge]][travis/latest]
[![Build Status master][travis/master/badge]][travis/master]
[![Tag][github/tags/badge]][github/tags]
[![Crates.io][crates.io/osmgraphing/badge]][crates.io/osmgraphing]
[![Docs][docs.rs/osmgraphing/badge]][docs.rs/osmgraphing]

[![License][github/license/badge]][github/license]
[![Last commit][github/last-commit/badge]][github/last-commit]

Goal of this student project is parsing [openstreetmap][osm] data to calculate traffic routes on it.

## News

The automatic deployment to [crates.io][crates.io/osmgraphing] is working.
The parser has been finished and can parse Germany in ±8 minutes on a common machine.

The `braess`-simulation has been implemented.
It tries to find potential bottlenecks in street-networks.
The idea is to calculate some routes via selfish routing (e.g. fastest path), resulting in routes and theoretical costs.
With these selfish-routes, actual route-costs are calculated.
These actual costs could be very different to the theoretical costs, since selfish routing leads to bad coverage of the streetgraph.
The goal is to reduce the actual costs by removing edges from the streetgraph using the number of routes per edge.
The implemented solution runs concurrently and shows some nice issues, but should be seen as starting-point for further digging.

Next steps will be cleaning up a little and doing the master-thesis until submission at `June 19th, 2020`.
It will play around with new metrices basing on edge-usages after calculating alternative routes.

## Setup and usage

Please refer to [setup and usage][github/usage] to get details about the project setup and how to run the code.

[crates.io/osmgraphing/badge]: https://img.shields.io/crates/v/osmgraphing
[crates.io/osmgraphing]: https://crates.io/crates/osmgraphing
[docs.rs/osmgraphing/badge]: https://docs.rs/osmgraphing/badge.svg
[docs.rs/osmgraphing]: https://docs.rs/osmgraphing/
[github/last-commit/badge]: https://img.shields.io/github/last-commit/dominicparga/osmgraphing
[github/last-commit]: https://github.com/dominicparga/osmgraphing/commits
[github/license/badge]: https://img.shields.io/github/license/dominicparga/osmgraphing
[github/license]: https://github.com/dominicparga/osmgraphing/blob/master/LICENSE
[github/usage]: https://github.com/dominicparga/osmgraphing/wiki/Usage
[github/tags/badge]: https://img.shields.io/github/v/tag/dominicparga/osmgraphing?sort=semver
[github/tags]: https://github.com/dominicparga/osmgraphing/tags
[osm]: https://openstreetmap.org
[travis/latest/badge]: https://img.shields.io/travis/com/dominicparga/osmgraphing?label=latest%20build
[travis/latest]: https://travis-ci.com/dominicparga/osmgraphing
[travis/master/badge]: https://img.shields.io/travis/com/dominicparga/osmgraphing/master?label=master-build
[travis/master]: https://travis-ci.com/dominicparga/osmgraphing/branches
