.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams graph
	substreams info

.PHONY: cache
cache:
	substreams-sink-noop optimism.substreams.pinax.network:443 contract-creation-block-index-v0.2.0.spkg map_block_index 0:

.PHONY: gui
gui:
	substreams gui contract-creation-block-index-v0.2.0.spkg -e optimism.substreams.pinax.network:443 map_block_index -s 128205467 -t 128245467 --network optimism