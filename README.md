# xdpdrop

Drop IPv4 packets using XDP (eXpress Data Path), an eBPF-based high-performance data path used to send and receive network packets at high rates.

## Configuration file

A YAML file with the list of IPs to filter:

```
ipv4s:
  - 8.8.8.8
  - 1.1.1.1
```

## Usage

```
make build
sudo target/debug/xdpdrop --file your_list_of_ips.yaml
```

## TODO

- IPV6 drop
- DNS drop
