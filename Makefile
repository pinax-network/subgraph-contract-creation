.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info
	substreams graph

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: gui
gui:
	substreams gui . -e eth.substreams.pinax.network:443 map_contract_creation -s 20444295 --network mainnet

.PHONY: parquet
parquet:
	substreams-sink-files run eth.substreams.pinax.network:443 substreams.yaml map_contract_creation './out' 20444295:20444795 --encoder parquet --file-block-count 100 --development-mode
