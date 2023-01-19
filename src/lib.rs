use std::{net::Ipv4Addr, str::FromStr};

use anyhow::{bail, Result};

mod config;
mod ipdrop;
mod xdp;

use config::Config;

fn bump_memlock_rlimit() -> Result<()> {
    let rlimit = libc::rlimit {
        rlim_cur: 128 << 20,
        rlim_max: 128 << 20,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
        bail!("Failed to increase rlimit");
    }

    Ok(())
}

pub fn drop(config_file: &str) -> Result<()> {
    let conf = Config::new(&config_file).unwrap();

    let ipv4s = conf
        .ipv4s
        .iter()
        .map(|ip| Ipv4Addr::from_str(ip).unwrap())
        .collect();

    bump_memlock_rlimit()?;

    ipdrop::drop_ipv4_packets(ipv4s)
}
