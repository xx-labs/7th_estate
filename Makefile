SEVENTH_ESTATE_BINARY="./seventh-estate"
OUTFOLDER="data"
INFOLDER="examples"
SIZE=32

install:
	cargo build
	cp target/debug/seventh-estate .

run:

	mkdir ${OUTFOLDER} 2>/dev/null
	cp ${INFOLDER}/newpoll.yaml ${OUTFOLDER}/example.yaml
	cp ${INFOLDER}/roster.csv ${OUTFOLDER}/roster.csv
	cp ${INFOLDER}/xxn.yaml ${OUTFOLDER}/xxn.yaml
	cp ${INFOLDER}/serials.csv ${OUTFOLDER}/serials.csv


	${SEVENTH_ESTATE_BINARY} new --config ${OUTFOLDER}/example.yaml
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.new

	${SEVENTH_ESTATE_BINARY} bind-roster --config ${OUTFOLDER}/example.yaml.secure --roster ${OUTFOLDER}/roster.csv
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.bind

	${SEVENTH_ESTATE_BINARY} step1 --config ${OUTFOLDER}/example.yaml.secure
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.step1

	${SEVENTH_ESTATE_BINARY} step2 --config ${OUTFOLDER}/example.yaml.secure --seed `openssl rand ${SIZE} | xxd -ps -c ${SIZE}`
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.step2

	${SEVENTH_ESTATE_BINARY} step3 --config ${OUTFOLDER}/example.yaml.secure --addresses ${OUTFOLDER}/addresses.csv --ballots ${OUTFOLDER}/ballots.csv
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.step3

	${SEVENTH_ESTATE_BINARY} step4 --config ${OUTFOLDER}/example.yaml.secure --serial-file ${OUTFOLDER}/serials.csv --xxn ${OUTFOLDER}/xxn.yaml
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.step4

finnish:
	cp ${INFOLDER}/votes.csv ${OUTFOLDER}/votes.csv

	${SEVENTH_ESTATE_BINARY} step5
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.step5

	${SEVENTH_ESTATE_BINARY} step6 --config ${OUTFOLDER}/example.yaml.secure --votes ${OUTFOLDER}/votes.csv
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.step6

	${SEVENTH_ESTATE_BINARY} step7 --config ${OUTFOLDER}/example.yaml.secure --seed `openssl rand ${SIZE} | xxd -ps -c ${SIZE}`
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.step7

	${SEVENTH_ESTATE_BINARY} step8 --config ${OUTFOLDER}/example.yaml.secure
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.step8

	${SEVENTH_ESTATE_BINARY} audit --config ${OUTFOLDER}/example.yaml.secure  --xxn ${OUTFOLDER}/xxn.yaml
	cp ${OUTFOLDER}/example.yaml.secure ${OUTFOLDER}/example.yaml.secure.audit
	
clean:
	rm -rf data
	rm -rf merkle.yaml
	rm -rf ballots
	rm -rf Example\ Poll

