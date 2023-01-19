#include "vmlinux.h"
#include <bpf/bpf_endian.h>
#include <bpf/bpf_helpers.h>

#define ETH_P_IP        0x0800

struct {
        __uint(type, BPF_MAP_TYPE_HASH);
        __type(key, __u32);
        __type(value, __u32);
        __uint(max_entries, 4096);
} source_ips SEC(".maps");


SEC("xdp")
int xdp_drop_prog(struct xdp_md *ctx)
{
    // start and end of packet data
    void *data = (void *)(long)ctx->data;
    void *data_end = (void *)(long)ctx->data_end;

    // counter map values
    __u32 *value;

    struct ethhdr *eth = data;
    if (data + sizeof(struct ethhdr) > data_end)
        return XDP_ABORTED;

    if (eth->h_proto != bpf_htons(ETH_P_IP))
        return XDP_PASS;

    struct iphdr *iph = data + sizeof(struct ethhdr);
    if (data + sizeof(struct ethhdr) + sizeof(struct iphdr) > data_end)
        return XDP_ABORTED;


    __u32 ip_src = iph->saddr;
    value = bpf_map_lookup_elem(&source_ips, &ip_src);
    if (value) {
        bpf_printk("[RS] Dropping packet from source_ips map %x", iph->saddr);
        *value += 1;
        return XDP_ABORTED;
    }

    return XDP_PASS;
};

char _license[] SEC("license") = "GPL";
