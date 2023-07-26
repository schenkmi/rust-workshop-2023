#!/bin/bash
set -e

(cd rust-lib && cargo test && cargo build)
gcc -o rust-in-c main.c rust-lib/target/debug/libprime_numbers.a
./rust-in-c
