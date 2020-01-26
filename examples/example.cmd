cd ..
mkdir data
copy examples\newpoll.yaml data\example.yaml
copy examples\roster.csv data\roster.csv

seventh-estate.exe new --config data\example.yaml
seventh-estate.exe bind-roster --config data\example.yaml.secure --roster data\roster.csv
seventh-estate.exe step1 --config data\example.yaml.secure
seventh-estate.exe step2 --config data\example.yaml.secure --seed 00112233445566778899aabbccddeeffffeeddccbbaa99887766554433221100
seventh-estate.exe step3 --config data\example.yaml.secure --addresses addresses.csv --ballots ballots.csv
seventh-estate.exe step4 --config data\example.yaml.secure --serial-file serials.csv
seventh-estate.exe step5
seventh-estate.exe step6 --config data\example.yaml.secure --votes votes.csv
seventh-estate.exe step7 --config data\example.yaml.secure --seed ffeeddccbbaa9988776655443322110000112233445566778899aabbccddeeff
seventh-estate.exe step8 --config data\example.yaml.secure
