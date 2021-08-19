#!/bin/sh
SEVENTH_ESTATE_BINARY="./seventh-estate"
OUTFOLDER="data"


mkdir $OUTFOLDER
cp examples/newpoll.yaml ${OUTFOLDER}/example.yaml
cp examples/roster.csv ${OUTFOLDER}/roster.csv

${SEVENTH_ESTATE_BINARY} new --config ${OUTFOLDER}/example.yaml
${SEVENTH_ESTATE_BINARY} bind-roster --config ${OUTFOLDER}/example.yaml.secure --roster ${OUTFOLDER}/roster.csv
${SEVENTH_ESTATE_BINARY} step1 --config ${OUTFOLDER}/example.yaml.secure
${SEVENTH_ESTATE_BINARY} step2 --config ${OUTFOLDER}/example.yaml.secure --seed 00112233445566778899aabbccddeeffffeeddccbbaa99887766554433221100
${SEVENTH_ESTATE_BINARY} step3 --config ${OUTFOLDER}/example.yaml.secure --addresses addresses.csv --ballots ballots.csv
${SEVENTH_ESTATE_BINARY} step4 --config ${OUTFOLDER}/example.yaml.secure --serial-file serials.csv
# ${SEVENTH_ESTATE_BINARY} step5
# ${SEVENTH_ESTATE_BINARY} step6 --config ${OUTFOLDER}/example.yaml.secure --votes votes.csv
# ${SEVENTH_ESTATE_BINARY} step7 --config ${OUTFOLDER}/example.yaml.secure --seed ffeeddccbbaa9988776655443322110000112233445566778899aabbccddeeff
# ${SEVENTH_ESTATE_BINARY} step8 --config ${OUTFOLDER}/example.yaml.secure
