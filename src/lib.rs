use std::net::Ipv4Addr;

use anyhow::{bail, Result};

mod config;
mod drop;
mod xdp;

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

pub fn drop(interface: &str, ips_file: Option<String>, dns_file: Option<String>) -> Result<()> {
    let ifidx = unsafe { libc::if_nametoindex(interface.into_raw()) };

    bump_memlock_rlimit()?;

    let ctrl_c_events = ctrl_channel()?;
    let mut ips: Vec<Ipv4Addr> = Vec::new();
    let mut dns: Vec<String> = Vec::new();

    if let Some(ips_file) = ips_file {
        ips = config::iplist::from_file(&ips_file)?
    }

    if let Some(dns_file) = dns_file {
        dns = config::dnslist::from_file(&dns_file)?
    }

    drop::xdp_drop(ips, dns, ctrl_c_events)
}
