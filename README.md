# xdpdrop

Drop networking packets of a specific IPv4 or DNS domain using XDP (eXpress Data Path), an eBPF-based high-performance data path used to send and receive network packets at high rates.

## Configuration file

A YAML file with the list of IPs to filter:

```
ipv4s:
  - 8.8.8.8
  - 1.1.1.1
dns:
  - amazon.es
  - microbit.org
```

## Usage

```
make build
sudo target/debug/xdpdrop --file your_list_of_ips.yaml

// ping amazon.es
// cat /sys/kernel/debug/tracing/trace_pipe
```

## TODO

- IPV6 drop
