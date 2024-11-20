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
	substreams-sink-noop eth.substreams.pinax.network:443 ../../ graph_out 0:

.PHONY: publish
publish:
	make build
	graph publish --subgraph-id EEWpgBNwW9TzJeHppqyGfRxeK6UuXErg3VVn6EoWjsuc
