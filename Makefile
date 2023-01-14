build: src/bpf/xdpdrop.bpf.c
	cargo build
.PHONY: build

run: build
	sudo target/debug/xdpdrop
.PHONY: build
