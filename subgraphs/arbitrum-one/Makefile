.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node=${GRAPH_NODE_URL} contract_creation 
	graph deploy --node=${GRAPH_NODE_URL} contract_creation --version-label=${SUBGRAPH_VERSION} --ipfs=${IPFS_ENDPOINT}

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id REPLACE_ME
