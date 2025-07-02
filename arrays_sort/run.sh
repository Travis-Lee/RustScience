#!/usr/bin/env bash

rm -rf target Cargo.lock

cargo clean

cargo build --release 

cargo run --release

