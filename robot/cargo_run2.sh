#!/bin/sh
mkfifo /tmp/robot-fifo-0
mkfifo /tmp/robot-fifo-1
x-terminal-emulator -e sh -c "RUST_LOG=robot=info cargo run  -- --input /tmp/robot-fifo-0 --output /tmp/robot-fifo-1 --name 1" &
x-terminal-emulator -e sh -c "RUST_LOG=robot=info cargo run  -- --input /tmp/robot-fifo-1 --output /tmp/robot-fifo-0 --name 2" &

wait
