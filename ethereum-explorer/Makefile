ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 17717997
#17712040
#17717997
STOP_BLOCK ?= +50

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_transfers -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: stream1
stream1: build
	substreams run -e $(ENDPOINT) substreams.yaml map_events -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: stream2
stream2: build
	substreams run -e $(ENDPOINT) substreams.yaml map_block_meta -s $(START_BLOCK) -t $(STOP_BLOCK)


.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: stream
package: build
	substreams package substreams.yaml
