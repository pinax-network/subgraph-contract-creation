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

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id REPLACE_ME
