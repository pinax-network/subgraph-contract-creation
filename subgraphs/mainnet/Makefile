.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	make build
	graph create contract_creation
	
	if [ -z "$(SUBGRAPH_VERSION)" ]; then \
		graph deploy contract_creation --version-label=$(SUBGRAPH_VERSION);
	else \
		echo "Getting version from git tag: $(shell git describe --tags --abbrev=0)"; \
		graph deploy contract_creation --version-label=$(shell git describe --tags --abbrev=0);
	fi

.PHONY: publish
publish:
	make build
	graph publish --subgraph-id EEWpgBNwW9TzJeHppqyGfRxeK6UuXErg3VVn6EoWjsuc
