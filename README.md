# xdpdrop

Drop networking packets of a specific IPv4 or DNS domain using XDP (eXpress Data Path), an eBPF-based high-performance data path used to send and receive network packets at high rates.

## Usage

```
make build
sudo target/debug/main --interface eth0 --dns-list ./examples/advertising.txt

// cat /sys/kernel/debug/tracing/trace_pipe
```

### DNS lists

Standard format for DNS blocklists files: https://github.com/blocklistproject/Lists

### IP lists

One IPv4 per line on the file.

## TODO

- IPV6 drop
