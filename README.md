# osmgraphing

[![Build Status][www_travis_builds_badge]][www_travis_builds]
[![Crates.io][www_crates_io]][www_crates_io]
[![Docs][www_docs_badge]][www_docs]
[![License][www_license_badge]][www_license]

Goal of this student project is parsing [openstreetmap][www_openstreetmap] data to calculate traffic routes on it.

## News

The automatic deployment to [crates.io][www_cratesio_osmgraphing] is working.
The parser has been finished and can parse Germany in ±8 minutes on a common machine.

Next step will be trying to find potential bottlenecks in street-networks.
The idea is to calculate some routes via selfish routing (e.g. fastest path), resulting in routes and theoretical costs.
With these selfish-routes, actual route-costs are calculated.
These actual costs could be very different to the theoretical costs, since selfish routing leads to bad coverage of the streetgraph.
The goal is to reduce the actual costs by removing edges from the streetgraph using the number of routes per edge.

Documentation and info follows. :)

## Setup and usage

Please refer to [setup and usage][www_osmgraphing_usage] to get details about the project setup and how to run the code.

[www_cratesio_osmgraphing]: https://crates.io/crates/osmgraphing

[www_travis_builds_badge]: https://travis-ci.com/dominicparga/osmgraphing.svg
[www_travis_builds]: https://travis-ci.com/dominicparga/osmgraphing

[www_crates_io]: https://img.shields.io/crates/v/osmgraphing

[www_docs_badge]: https://docs.rs/osmgraphing/badge.svg
[www_docs]: https://docs.rs/osmgraphing/

[www_license_badge]: https://img.shields.io/github/license/dominicparga/osmgraphing
[www_license]: https://github.com/dominicparga/osmgraphing/blob/master/LICENSE

[www_openstreetmap]: https://openstreetmap.org
[www_osmgraphing_usage]: https://github.com/dominicparga/osmgraphing/wiki/Usage
