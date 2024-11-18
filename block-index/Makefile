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
	substreams gui . -e arbone.substreams.pinax.network:443 map_block_index -s 0 -t 0 --network arbitrum-one --production-mode

.PHONY: gui
gui:
	substreams gui . -e eth.substreams.pinax.network:443 map_block_index -s 21186059 -t 21186069 --network mainnet