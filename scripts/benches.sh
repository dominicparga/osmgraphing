#!/usr/bin/env sh

# view the results in ./target/criterion/<bench>/report/index.html
cargo bench --bench routing -- --warm-up-time 10 --measurement-time 120
