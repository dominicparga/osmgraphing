# osmgraphing

[![Build Status][www_travis_builds_badge]][www_travis_builds]
[![Crates.io][www_crates_io]][www_crates_io]
[![Docs][www_docs_badge]][www_docs]
[![License][www_license_badge]][www_license]

Goal of this student project is parsing [openstreetmap][www_openstreetmap] data to calculate traffic routes on it.

## News

The automatic deployment to [crates.io][www_cratesio_osmgraphing] is working.

A lot of repo-work (creating issues, labels, deployments) has been done the past week.
Several branches contain great work (e.g. finished, but unoptimized parser), that has to be merged into the master.
This will be done immediately after the automated deployment is working.

After cleaning up and working off some issues (e.g. multi-stage-parsing), this project will try to find potential bottlenecks in street-networks.
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
