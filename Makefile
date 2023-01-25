build: src/bpf/xdpdrop.bpf.c
	cargo build
.PHONY: build

run: build
	sudo target/debug/main --interface enp0s5 --dns-list ./examples/advertising.txt --ip-list ./examples/tracking.ip
.PHONY: build
