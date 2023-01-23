use std::{net::Ipv4Addr, str::FromStr};

use anyhow::{bail, Result};

mod config;
mod drop;
mod xdp;

use config::Config;

use crossbeam_channel::{bounded, Receiver};

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

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

pub fn drop(config_file: &str) -> Result<()> {
    let conf = Config::new(&config_file).unwrap();

    let ipv4s = conf
        .ipv4s
        .iter()
        .map(|ip| Ipv4Addr::from_str(ip).unwrap())
        .collect();

    bump_memlock_rlimit()?;

    let ctrl_c_events = ctrl_channel()?;

    drop::xdp_drop(ipv4s, conf.dns, ctrl_c_events)
}
