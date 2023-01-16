use std::net::Ipv4Addr;

use anyhow::Result;

mod ipdrop;

fn main() -> Result<()> {
    let ips: &[Ipv4Addr] = &[
        Ipv4Addr::new(8, 8, 8, 8),
        Ipv4Addr::new(195, 76, 147, 109),
        Ipv4Addr::new(147, 83, 249, 103),
    ];
    ipdrop::drop_ipv4_packets(ips)
}
