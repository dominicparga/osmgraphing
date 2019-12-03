# osmgraphing

[![Build Status latest][www_travis_builds_latest_badge]][www_travis_builds_latest]
[![Build Status master][www_travis_builds_master_badge]][www_travis_builds_master]
[![Tag][www_tags_badge]][www_tags]
[![Crates.io][www_crates_io_badge]][www_crates_io]
[![Docs][www_docs_badge]][www_docs]

[![License][www_license_badge]][www_license]
[![Last commit][www_last_commit_badge]][www_last_commit]

Goal of this student project is parsing [openstreetmap][www_openstreetmap] data to calculate traffic routes on it.

## News

The automatic deployment to [crates.io][www_cratesio_osmgraphing] is working.
The parser has been finished and can parse Germany in ±8 minutes on a common machine.

The `braess`-simulation has been implemented.
It tries to find potential bottlenecks in street-networks.
The idea is to calculate some routes via selfish routing (e.g. fastest path), resulting in routes and theoretical costs.
With these selfish-routes, actual route-costs are calculated.
These actual costs could be very different to the theoretical costs, since selfish routing leads to bad coverage of the streetgraph.
The goal is to reduce the actual costs by removing edges from the streetgraph using the number of routes per edge.
The implemented solution runs concurrently and shows some nice issues, but should be seen as starting-point for further digging.

Next steps will be cleaning up a little, before the master-thesis starts.
Travis doesn't build properly due to timeouts, although the examples are running locally.
The wiki should explain more implementation-ideas and the visualization of the `braess`-simulation could be improved.
After cleanup, I will do my master-thesis with this project.
It will play around with new metrices basing on edge-usages after calculating alternative routes.

## Setup and usage

Please refer to [setup and usage][www_osmgraphing_usage] to get details about the project setup and how to run the code.

[www_cratesio_osmgraphing]: https://crates.io/crates/osmgraphing

[www_openstreetmap]: https://openstreetmap.org
[www_osmgraphing_usage]: https://github.com/dominicparga/osmgraphing/wiki/Usage

[www_travis_builds_latest_badge]: https://img.shields.io/travis/com/dominicparga/osmgraphing?label=latest%20build
[www_travis_builds_master_badge]: https://img.shields.io/travis/com/dominicparga/osmgraphing/master?label=master-build
[www_travis_builds_latest]: https://travis-ci.com/dominicparga/osmgraphing
[www_travis_builds_master]: https://travis-ci.com/dominicparga/osmgraphing/branches
[www_tags_badge]: https://img.shields.io/github/v/tag/dominicparga/osmgraphing?sort=semver
[www_tags]: https://github.com/dominicparga/osmgraphing/tags
[www_crates_io_badge]: https://img.shields.io/crates/v/osmgraphing
[www_crates_io]: https://crates.io/crates/osmgraphing
[www_docs_badge]: https://docs.rs/osmgraphing/badge.svg
[www_docs]: https://docs.rs/osmgraphing/
[www_license_badge]: https://img.shields.io/github/license/dominicparga/osmgraphing
[www_license]: https://github.com/dominicparga/osmgraphing/blob/master/LICENSE
[www_last_commit_badge]: https://img.shields.io/github/last-commit/dominicparga/osmgraphing
[www_last_commit]: https://github.com/dominicparga/osmgraphing/commits
