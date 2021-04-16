#!/bin/sh
SEVENTH_ESTATE_BINARY="cargo run"

mkdir data
cp examples/newpoll.yaml data/example.yaml
cp examples/roster.csv data/roster.csv

${SEVENTH_ESTATE_BINARY} new --config data/example.yaml
${SEVENTH_ESTATE_BINARY} bind-roster --config data/example.yaml.secure --roster data/roster.csv
${SEVENTH_ESTATE_BINARY} step1 --config data/example.yaml.secure
${SEVENTH_ESTATE_BINARY} step2 --config data/example.yaml.secure --seed 00112233445566778899aabbccddeeffffeeddccbbaa99887766554433221100
${SEVENTH_ESTATE_BINARY} step3 --config data/example.yaml.secure --addresses addresses.csv --ballots ballots.csv
# ${SEVENTH_ESTATE_BINARY} step4 --config data/example.yaml.secure --serial-file serials.csv
# ${SEVENTH_ESTATE_BINARY} step5
# ${SEVENTH_ESTATE_BINARY} step6 --config data/example.yaml.secure --votes votes.csv
# ${SEVENTH_ESTATE_BINARY} step7 --config data/example.yaml.secure --seed ffeeddccbbaa9988776655443322110000112233445566778899aabbccddeeff
# ${SEVENTH_ESTATE_BINARY} step8 --config data/example.yaml.secure
