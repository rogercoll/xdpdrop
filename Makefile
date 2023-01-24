build: src/bpf/xdpdrop.bpf.c
	cargo build
.PHONY: build

run: build
	sudo target/debug/main --dns-list ./examples/advertising.txt --ip-list ./examples/tracking.ip
.PHONY: build
