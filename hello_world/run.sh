#!/usr/bin/env bash

rm -rf target Cargo.lock

cargo build --release 

cargo run --release

