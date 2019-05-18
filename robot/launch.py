#!/usr/bin/env python3

import subprocess
import sys
import argparse
from argparse import ArgumentParser, ArgumentDefaultsHelpFormatter


def sh(command):
    if isinstance(command, str):
        command = command.split()
    print(*command)
    out = subprocess.run(
        command, stdout=subprocess.PIPE, stderr=sys.stderr, encoding='utf-8'
    )
    return out.stdout


def main(base_command: str, count: int, loglevel: str, cargoargs):
    for i in range(count):
        sh(f'mkfifo /tmp/robot-fifo-{i}')

    for i in range(count):
        j = (i + 1) % count
        sh(
            [
                'x-terminal-emulator',
                '-e',
                # base_command.format(ARGS='--release' if release else '', IN=f"/tmp/robot-fifo-{i}", OUT=f"/tmp/robot-fifo-{j}", NAME=i),
                base_command.format(LOGLVL=loglevel, ARGS=str.join(' ', cargoargs), IN=f"/tmp/robot-fifo-{i}", OUT=f"/tmp/robot-fifo-{j}", NAME=i),
            ]
        )


if __name__ == '__main__':
    p = ArgumentParser(
        description="Generate a network named pipes (FIFO files) and launch the applications.",
        formatter_class=ArgumentDefaultsHelpFormatter,
    )
    p.add_argument(
        'count', type=int, help="The number of nodes in the network (must be >= 2)"
    )
    p.add_argument(
        'loglevel',
        type=str,
        default='trace',
        help="set the debug level",
    )
    p.add_argument(
        'cargoargs',
        nargs=argparse.REMAINDER,
        help="args forwarded to cargo",
    )

    base_command ='sh -c "RUST_LOG=robot={LOGLVL} cargo run {ARGS} -- --input {IN} --output {OUT} --name {NAME}"'
    args = p.parse_args()
    assert args.count >= 2, "The number of nodes must be >= 2"
    main(base_command, args.count, args.loglevel, args.cargoargs)
