use std::{net::Ipv4Addr, str::FromStr};

use anyhow::Result;

mod config;
mod ipdrop;

use config::Config;

pub fn drop(config_file: &str) -> Result<()> {
    let conf = Config::new(&config_file).unwrap();

    let ipv4s = conf
        .ipv4s
        .iter()
        .map(|ip| Ipv4Addr::from_str(ip).unwrap())
        .collect();

    ipdrop::drop_ipv4_packets(ipv4s)
}
