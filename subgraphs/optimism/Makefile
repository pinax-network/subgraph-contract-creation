.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node=http://localhost:8020 contract_creation
	graph deploy --node=http://localhost:8020 contract_creation

.PHONY: cache
cache:
	substreams-sink-noop optimism.substreams.pinax.network:443 ../../ graph_out 105239153:

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id 7RdSF6qXBh1HPFD9A9QV6UvqKCZhrWCXAsGWnCYaWeWE
