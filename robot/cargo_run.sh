#!/bin/sh

RUST_LOG=robot=trace cargo run --color=always $@ -- -i fifo -o fifo
