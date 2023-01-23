build: src/bpf/xdpdrop.bpf.c
	cargo build
.PHONY: build

run: build
	sudo target/debug/main --file ./examples/ipv4_drop.yaml
.PHONY: build
