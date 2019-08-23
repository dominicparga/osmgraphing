# osmgraphing

[![Build Status][www_travis_builds_badge]][www_travis_builds]
[![Crates.io][www_crates_io]][www_crates_io]
[![Docs][www_docs_badge]][www_docs]

Goal of this student project is parsing [openstreetmap][www_openstreetmap] data to calculate traffic routes on it.

## News

The project setup has been finished and examples can be added easily, which helps a lot when implementing.
The parser is able to parse pbf-files and currently just prints them.
The routing kind of works, but it has to get aligned with the parser.

Now, the bridge between parser and routing is in creation-process by @dominicparga.
Further, we want to visualize the routing via a small Rust-backend/JS-frontend application, where @PraiseTheFun is working on.

We have to submit our project in 1 week and the university is quite stressful these weeks, but test cases (and probably needed cleanup) will follow. :)

## Setup and usage

Please refer to [setup and usage][www_osmgraphing_usage] to get details about the project setup and how to run the code.

[www_travis_builds_badge]: https://travis-ci.com/dominicparga/osmgraphing.svg?branch=master
[www_travis_builds]: https://travis-ci.com/dominicparga/osmgraphing

[www_crates_io]: https://img.shields.io/crates/v/osmgraphing

[www_docs_badge]: https://docs.rs/osmgraphing/badge.svg
[www_docs]: https://docs.rs/osmgraphing/

[www_openstreetmap]: https://openstreetmap.org
[www_osmgraphing_usage]: https://github.com/dominicparga/osmgraphing/wiki/Usage
