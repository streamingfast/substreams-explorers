.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google,sf/bitcoin"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: package
package: build
	substreams pack substreams.yaml
