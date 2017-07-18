#!/bin/sh
cd "$(readlink -e "$(dirname "$0")")"
RUSTFLAGS='-Zalways-encode-mir' xargo build
