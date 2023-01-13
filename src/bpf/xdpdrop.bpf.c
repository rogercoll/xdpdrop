#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <linux/if_ether.h>
#include <arpa/inet.h>
#include <linux/ip.h>

#define DROP_IP_ADDRESS (unsigned int)(147 + (83 << 8) + (249 << 16) + (103 << 24))

struct {
        __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
        __type(key, __u32);
        __type(value, long);
        __uint(max_entries, 1);
} counter SEC(".maps");

SEC("xdp")
int xdp_drop_prog(struct xdp_md *ctx)
{
    // start and end of packet data
    void *data = (void *)(long)ctx->data;
    void *data_end = (void *)(long)ctx->data_end;

    // counter map values
    __u32 key = 0;
    long *value;

    struct ethhdr *eth = data;
    if (data + sizeof(struct ethhdr) > data_end)
        return XDP_ABORTED;

    if (ntohs(eth->h_proto) != ETH_P_IP)
        return XDP_PASS;

    struct iphdr *iph = data + sizeof(struct ethhdr);
    if (data + sizeof(struct ethhdr) + sizeof(struct iphdr) > data_end)
        return XDP_ABORTED;


    if (iph->saddr == DROP_IP_ADDRESS)
    {
        bpf_printk("[RS] Dropping packet from %x", iph->saddr);
        value = bpf_map_lookup_elem(&counter, &key);
        if (value)
            *value += 1;
        return XDP_ABORTED;
    }

    return XDP_PASS;
};

char _license[] SEC("license") = "GPL";
