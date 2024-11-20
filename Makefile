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
	substreams gui . -e optimism.substreams.pinax.network:443 map_contract_creation -s 128245467 --network optimism

.PHONY: parquet
parquet:
	substreams-sink-files run optimism.substreams.pinax.network:443 substreams.yaml map_contract_creation './out' 128245467:128245967 --encoder parquet --file-block-count 100 --development-mode
