# Seventh Estate: Grassroots Democracy

## Overview
Seventh Estate empowers grassroots democracy. With modern advances in
cryptography, anyone can now prove that a majority of the country
agrees with a particular petition statement (assuming that it is
indeed popular).  The technique offers
the best of surveys, initiatives, juries, protests, and petitions,
while overcoming their shortcomings in demonstrating the public will.

With it, anyone can now for the first time provide comprehensive evidence
that a majority would sign on to particular petition language.  All that’s
needed is about $1,000 worth of supplies and a postal mailing list
for the country or region.

The list is sampled in a way that is irrefutably random and
un-manipulatable.  Those receiving ballots are given time to research
and submit their decision securely online. They know that their input
is significant because of the limited sample size. They also know
that it will be counted correctly, because the security is
far superior to that of conventional election systems. In fact,
observers can verify that those creating and mailing the paper
ballots cannot influence selection, manipulate outcomes, or link
responses to addresses. Vote-buying, an unsolved problem in all other
current non-polling-place balloting, is effectively solved by *decoy ballots*.
Economic incentives can be provided to those casting ballots
without any possible linkage to their response.

The needed supplies are readily available and have already been
tested, the server backend can be part of any blockchain, and the
poll can be announced only after the ballots are mailed, making
interference all but impossible.

For a more complete introduction, see the
[Grassroots Democracy Booklet](Grassroots%20Democracy%20Booklet.pdf).

For an explanatory diagram and specifics on how it works, see the
[7th Estate Technical Diagram Explanation](7th%20estate%20technical%20description%20v1.pdf)

To learn much more about the underlying cryptography, statistics, proofs
and other background, see closely related ideas developed as part of
[Random-Sample Voting](https://rsvoting.org/):

* [Random-Sample Voting - More democratic, better quality, and far lower cost](https://rsvoting.org/whitepaper/white_paper.pdf) White Paper by David Chaum
* [Random Sample Voting: Security Proof](https://rsvoting.org/cryptographic_protocols/proof_summary.pdf),
Editors: David Chaum, Aggelos Kiayias, Douglas Wikström, Bingsheng, Zhang Summary: Jeremy Clark
* [Verifable Randomness Pillar Technical Summary](https://rsvoting.org/random_beacon/random_beacon_summary.pdf),
by Christopher Vatcher, edited by Alan Sherman and Aggelos Kiayias
* [Thwarting Vote Selling](https://docs.google.com/document/d/1a3Vz7O6RsFlQC1Z9u0ytUMqwk3MyMo2B8UCq5VZ5VIU/edit?usp=sharing),
by David C. Parkes, Paul Tylkin, Lirong Xia
* [Published Encrypted Rosters: Technical Summary](https://rsvoting.org/auditable_roster/auditable_roster_summary.pdf),
by David Chaum, Jeremy Clark, Neal McBurnett, Nan Yang
* [Real World Crypto 2016 Trial Report](https://github.com/rsvoting/publications/blob/master/trials/rwc-2016-demo-report.md)
* [Crypto 2015 Trial Report](https://github.com/rsvoting/publications/blob/master/trials/crypto-2015-demo-report.md)

## Installation

Clone this repository as described at
[Cloning a repository \- GitHub Docs](https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/cloning-a-repository)

You'll also need to install the Rust language, as explained at
[Install Rust \- Rust Programming Language](https://www.rust-lang.org/tools/install)

Next, run `cargo build`. This will install all the dependencies, and
compile them into an executable in `target/debug/seventh-estate`.

Run `target/debug/seventh-estate --help` to confirm that it built
successfully and learn the command-line options available.

## Run the Demo

The following commands work from the command line in a Linux terminal.
(On Windows, the commands in `examples/example.cmd` will be helpful.)

Change directory to the `examples` folder, and run

`SEVENTH_ESTATE_BINARY=../target/debug/seventh-estate ./example-debug.sh 2>&1 | tee example.out`

The output is shown in the console, and also captured in the file `example.out`.

When it asks for a password, for the demo, you can just make one up
and enter it consistently.

## Interpreting the results of the demo
Watch the Seventh Estate [Demonstration Video](https://youtu.be/v20n5pXAcvQ) for
an explanation of the steps in a poll.

Briefly, here is the sequence of events, identified by the
`seventh-estate` command used, and which files to look at for
results.

```
new: Create a new poll
 examples/example.yaml.secure (changed in later steps also)

bind-roster: Bind the roster of voters to the poll
 examples/example.yaml.secure

step 1: Generate initial commitments
 examples/ExamplePoll/committed_plane_*.csv
 examples/ExamplePoll/committed_roster.csv
 examples/ExamplePoll/committed_summands.yaml

step 2: Generate drawn summands
 examples/ExamplePoll/drawn_summands.yaml

step 3: Generate address labels and ballot information
 examples/addresses.csv
 examples/ballots.csv

step 4: Record audited (spoiled) ballots
 examples/ExamplePoll/print_audit_plane_*.csv
 examples/ExamplePoll/print_audit_plane_*_keys.csv

step 5: Ballot casting (the demo script chooses random votes)
 examples/votes.csv

step 6: Record votes
 examples/ExamplePoll/vote_plane_*.csv
 examples/ExamplePoll/vote_plane_*_keys.csv

step 7: Tally audit draw
 examples/ExamplePoll/audited_columns.yaml

step 8: Reveal tally and audit it
 examples/ExamplePoll/final_plane_*.csv
 examples/ExamplePoll/final_plane_*_keys.csv
 examples/ExamplePoll/committed_summands_key.key
 examples/ExamplePoll/committed_summands_revealed.csv
```
