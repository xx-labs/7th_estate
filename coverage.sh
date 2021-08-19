#!/bin/bash

export RUSTC_BOOTSTRAP=1
export RUSTFLAGS="-Zinstrument-coverage"
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

echo "[+] Building"
cargo build

echo "[+] Running tests"
cargo test


echo "[+] Generating report"
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/


echo "[+] Done"
