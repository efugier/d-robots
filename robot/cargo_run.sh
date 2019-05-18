#! /bin/sh

RUST_LOG=robot=trace cargo run --color=always ${@:1:99} -- -i fifo -o fifo
