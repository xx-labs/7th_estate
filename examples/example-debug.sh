#!/bin/bash
# To capture it all, run via SEVENTH_ESTATE_BINARY=../target/debug/seventh-estate ./example-debug.sh 2>&1 | tee example.out

# set -e # exit on error
set -u # exit on unset
set -v # verbose

function recordstep () {
    git add .;
    git commit -m "$1";
}

SEVENTH_ESTATE_BINARY=${SEVENTH_ESTATE_BINARY:-./seventh-estate}

cp one-trustee-poll.yaml example.yaml
cp roster.csv example-roster.csv

${SEVENTH_ESTATE_BINARY} new --config example.yaml
recordstep new
${SEVENTH_ESTATE_BINARY} bind-roster --config example.yaml.secure --roster example-roster.csv
recordstep bind-roster
${SEVENTH_ESTATE_BINARY} step1 --config example.yaml.secure
recordstep step1
${SEVENTH_ESTATE_BINARY} step2 --config example.yaml.secure --seed 00112233445566778899aabbccddeeffffeeddccbbaa99887766554433221100
recordstep step2
${SEVENTH_ESTATE_BINARY} step3 --config example.yaml.secure --addresses addresses.csv --ballots ballots.csv
recordstep step3
${SEVENTH_ESTATE_BINARY} step4 --config example.yaml.secure --serial-file serials.csv
recordstep step4

# Record some votes at random
#  Pick the first three "For" votes (lines 2-4)
#  Pick both options in ballots 4 and 5
#  Pick "Against" votes from ballots 6-10.
(echo votecode; cut -d, -f 2 ballots.csv| sed -n '2,6p'; cut -d, -f 4 ballots.csv| sed -n '5,11p'; ) > votes.csv

cat <<EOF

Time to simulate actual voting.  The votecodes are random, so this can't be predetermined.
Choose some actual votecodes for this election from ballots.csv
with some from column 2 and a different number from column 4,
so you don't end up with a tie"
Make sure you avoid the ones you've spoiled, and have enough to deal with random decoys.
Some have been prechosen for you, but you can edit votes.csv now if you like.

EOF

echo "Modify votes.csv if desired, and hit Enter when ready"
read ok

recordstep votes

# to see selected options in regular terminal: fgrep -n -f votes.csv ballots.csv

${SEVENTH_ESTATE_BINARY} step5
recordstep step5
${SEVENTH_ESTATE_BINARY} step6 --config example.yaml.secure --votes votes.csv
recordstep step6
${SEVENTH_ESTATE_BINARY} step7 --config example.yaml.secure --seed ffeeddccbbaa9988776655443322110000112233445566778899aabbccddeeff
recordstep step7
${SEVENTH_ESTATE_BINARY} step8 --config example.yaml.secure
recordstep step8
