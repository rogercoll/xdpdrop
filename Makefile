xdp-c/xdp_drop.o: xdp-c/xdp_drop.c
	clang -O2 -g -Wall -target bpf -c ./xdp-c/xdp_drop.c -o ./xdp-c/xdp_drop.o


load: xdp-c/xdp_drop.o
	sudo xdp-loader load -m skb -s xdp_drop wlp58s0 ./xdp-c/xdp_drop.o
.PHONY: load

unload:
	sudo xdp-loader unload -a wlp58s0
.PHONY: unload
