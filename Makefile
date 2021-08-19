SEVENTH_ESTATE_BINARY="./seventh-estate"
OUTFOLDER="data"
INFOLDER="examples"

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
	${SEVENTH_ESTATE_BINARY} bind-roster --config ${OUTFOLDER}/example.yaml.secure --roster ${OUTFOLDER}/roster.csv
	${SEVENTH_ESTATE_BINARY} step1 --config ${OUTFOLDER}/example.yaml.secure
	${SEVENTH_ESTATE_BINARY} step2 --config ${OUTFOLDER}/example.yaml.secure --seed 00112233445566778899aabbccddeeffffeeddccbbaa99887766554433221100
	${SEVENTH_ESTATE_BINARY} step3 --config ${OUTFOLDER}/example.yaml.secure --addresses ${OUTFOLDER}/addresses.csv --ballots ${OUTFOLDER}/ballots.csv
	${SEVENTH_ESTATE_BINARY} step4 --config ${OUTFOLDER}/example.yaml.secure --serial-file ${OUTFOLDER}/serials.csv --xxn ${OUTFOLDER}/xxn.yaml

clean:
	rm -rf data
	rm -rf merkle.yaml
	rm -rf ballots
	rm -rf Example\ Poll

