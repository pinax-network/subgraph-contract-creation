.PHONY: all
all:
	make block-index
	make spkg-build
	make subgraph-build-all

.PHONY: spkg-build
spkg-build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info
	substreams graph

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: gui
gui:
	substreams gui . -e eth.substreams.pinax.network:443 graph_out -s 21186059 -t 21186069 --network mainnet

.PHONY: block-index
block-index:
	make -C block-index

.PHONY: subgraph-build-%
null  :=
space := $(null) #
comma := ,
SUBDIRS := $(notdir $(wildcard subgraphs/*))
subgraph-build-%:
	$(eval TARGET := subgraphs/$*)
	$(eval VALID_TARGETS := $(subst $(space),$(comma)$(space),$(strip $(addprefix ',$(addsuffix ',$(SUBDIRS))))))

	@if [ "$*" = "all" ]; then \
		echo "Building ALL subgraphs $(VALID_TARGETS)..."; \
		make $(addprefix subgraph-build-, $(SUBDIRS)); \
	elif [ ! -d "$(TARGET)" ]; then \
		echo "'$*' is not a valid subgraph target. Valid targets are $(VALID_TARGETS)."; \
	else \
		echo "Building '$*'..."; \
		make -C $(TARGET); \
	fi
