.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

module ?= map_block_meta

.PHONY: run
run: build
	substreams run -e eth-cl.substreams.pinax.network\:443 $(filter-out $@,$(MAKECMDGOALS)) -s -10

.PHONY: gui
gui: build
	substreams gui -e eth-cl.substreams.pinax.network\:443 $(filter-out $@,$(MAKECMDGOALS)) -s -10