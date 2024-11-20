.PHONY: all
all:
	make subgraph-build-all

.PHONY: subgraph-build-%
null  :=
space := $(null) #
comma := ,
SUBDIRS := $(notdir $(wildcard *))
subgraph-build-%:
	$(eval TARGET := $*)
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
