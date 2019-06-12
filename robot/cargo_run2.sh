#!/bin/sh
mkfifo /tmp/robot-fifo-0
mkfifo /tmp/robot-fifo-1
xterm -e sh -c "RUST_LOG=robot=info cargo run --release -- --input /tmp/robot-fifo-0 --output /tmp/robot-fifo-1 --name 1; cat" &
xterm -e sh -c "RUST_LOG=robot=info cargo run --release -- --input /tmp/robot-fifo-1 --output /tmp/robot-fifo-0 --name 2 -x 0.5; cat" &

wait
