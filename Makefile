.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: gui
gui:
	substreams gui . -e eth.substreams.pinax.network:443 graph_out -s 21186059 -t 21186069 --network mainnet