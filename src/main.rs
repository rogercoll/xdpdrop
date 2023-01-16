use std::{net::Ipv4Addr, str::FromStr};

use anyhow::Result;

mod config;
mod ipdrop;

use argh::FromArgs;
use config::Config;

#[derive(FromArgs)]
/// xdpdrop CLI config
struct CLI {
    /// configuration file
    #[argh(option, short = 'f')]
    file: String,
}

fn main() -> Result<()> {
    let cli_conf: CLI = argh::from_env();

    let conf = Config::new(&cli_conf.file).unwrap();

    let ipv4s = conf
        .ipv4s
        .iter()
        .map(|ip| Ipv4Addr::from_str(ip).unwrap())
        .collect();

    ipdrop::drop_ipv4_packets(ipv4s)
}
