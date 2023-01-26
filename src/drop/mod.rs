use std::net::Ipv4Addr;
use std::time::Duration;

use anyhow::Result;

mod xdpdrop {
    include!(concat!(env!("OUT_DIR"), "/xdpdrop.skel.rs"));
}

use crossbeam_channel::{select, tick, Receiver};
use libbpf_rs::MapFlags;
use xdpdrop::*;

use crate::xdp::helpers::{attach_xdp_best_available, xdp_detach};

pub fn xdp_drop(
    device_id: i32,
    target_ips: Vec<Ipv4Addr>,
    target_dns: Vec<String>,
    done: Receiver<()>,
) -> Result<()> {
    let skel_builder = XdpdropSkelBuilder::default();
    // skel_builder.obj_builder.debug(true);
    let open_skel = skel_builder.open()?;

    let mut skel = open_skel.load()?;

    let mode = unsafe { attach_xdp_best_available(device_id, skel.progs_mut().xdp_drop().fd())? };

    let vals: u32 = 0;
    for dns in target_dns.iter() {
        let mut dns_array = [0u8; 128];
        dns_array[..dns.len()].copy_from_slice(dns.as_bytes());
        skel.maps_mut()
            .recordmap()
            .update(&dns_array, &vals.to_ne_bytes(), MapFlags::ANY)?;
    }

    for ip in target_ips.iter() {
        // IP address must be encoded as big endian notation
        let target = u32::from(*ip).to_be_bytes();
        skel.maps_mut()
            .source_ips()
            .update(&target, &vals.to_ne_bytes(), MapFlags::ANY)?;
    }

    println!("\nLoaded {} domain names", target_dns.len());
    println!("\nLoaded {} IPs", target_ips.len());
    println!("\nXDPDrop started");

    let ticks = tick(Duration::from_secs(10));
    loop {
        select! {
            recv(ticks) -> _ => {
                println!("Gathering stats");
            }
            recv(done) -> _ => {
                println!("Exiting, detaching dns xdp program");
                break;
            }
        }
    }
    unsafe { xdp_detach(device_id, mode) }
}
