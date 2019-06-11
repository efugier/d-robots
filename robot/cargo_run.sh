#!/bin/sh

RUST_LOG=robot=trace cargo run --release --color=always $@ -- -i fifo -o fifo -n 1
