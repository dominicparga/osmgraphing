# osmgraphing

[![Tag][github/dominicparga/osmgraphing/tags/badge]][github/dominicparga/osmgraphing/tags]
[![Crates.io][crates.io/osmgraphing/badge]][crates.io/osmgraphing]
[![Docs][docs.rs/osmgraphing/badge]][docs.rs/osmgraphing]

[![Build Status latest][travis/latest/badge]][travis/latest]
[![Build Status master][travis/master/badge]][travis/master]

[![Changelog][github/dominicparga/osmgraphing/blob/changelog/badge]][github/dominicparga/osmgraphing/blob/changelog]
[![Last commit][github/dominicparga/osmgraphing/last-commit/badge]][github/dominicparga/osmgraphing/last-commit]

[![License][github/dominicparga/osmgraphing/license/badge]][github/dominicparga/osmgraphing/license]

Goal of this repo is parsing [openstreetmap][osm] data to calculate traffic-routes and different related use-cases on it.


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

Please refer to [setup and usage][github/dominicparga/osmgraphing/usage] to get details about the project setup and how to run the code.


[crates.io/osmgraphing]: https://crates.io/crates/osmgraphing
[crates.io/osmgraphing/badge]: https://img.shields.io/crates/v/osmgraphing?style=for-the-badge
[docs.rs/osmgraphing]: https://docs.rs/osmgraphing/
[docs.rs/osmgraphing/badge]: https://img.shields.io/crates/v/osmgraphing?color=informational&label=docs&style=for-the-badge
[github/dominicparga/osmgraphing/blob/changelog]: https://github.com/dominicparga/osmgraphing/blob/master/CHANGELOG.md
[github/dominicparga/osmgraphing/blob/changelog/badge]: https://img.shields.io/badge/CHANGELOG-master-blueviolet?style=for-the-badge
[github/dominicparga/osmgraphing/last-commit]: https://github.com/dominicparga/osmgraphing/commits
[github/dominicparga/osmgraphing/last-commit/badge]: https://img.shields.io/github/last-commit/dominicparga/osmgraphing?style=for-the-badge
[github/dominicparga/osmgraphing/license]: https://github.com/dominicparga/osmgraphing/blob/master/LICENSE
[github/dominicparga/osmgraphing/license/badge]: https://img.shields.io/github/license/dominicparga/osmgraphing?style=for-the-badge
[github/dominicparga/osmgraphing/tags]: https://github.com/dominicparga/osmgraphing/tags
[github/dominicparga/osmgraphing/tags/badge]: https://img.shields.io/github/v/tag/dominicparga/osmgraphing?sort=semver&style=for-the-badge
[github/dominicparga/osmgraphing/usage]: https://github.com/dominicparga/osmgraphing/wiki/Usage
[osm]: https://openstreetmap.org
[travis/latest]: https://travis-ci.com/dominicparga/osmgraphing
[travis/latest/badge]: https://img.shields.io/travis/com/dominicparga/osmgraphing?label=latest%20build&style=for-the-badge
[travis/master]: https://travis-ci.com/dominicparga/osmgraphing/branches
[travis/master/badge]: https://img.shields.io/travis/com/dominicparga/osmgraphing/master?label=master-build&style=for-the-badge
