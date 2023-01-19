use std::{
    net::Ipv4Addr,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use anyhow::Result;

mod xdpdrop {
    include!(concat!(env!("OUT_DIR"), "/xdpdrop.skel.rs"));
}

use libbpf_rs::MapFlags;
use xdpdrop::*;

use crate::xdp::helpers::{attach_xdp_best_available, xdp_detach};

pub fn drop_ipv4_packets(target_ips: Vec<Ipv4Addr>) -> Result<()> {
    let mut skel_builder = XdpdropSkelBuilder::default();
    skel_builder.obj_builder.debug(true);
    let open_skel = skel_builder.open()?;

    // vars
    let interface_id = 2;

    // set ctrl handler to stop and unload the program
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })?;

    let mut skel = open_skel.load()?;

    let mode =
        unsafe { attach_xdp_best_available(interface_id, skel.progs_mut().xdp_drop_prog().fd())? };

    let vals: u32 = 0;
    for ip in target_ips.iter() {
        // IP address must be encoded as big endian notation
        let target = u32::from(*ip).to_be_bytes();
        skel.maps_mut()
            .source_ips()
            .update(&target, &vals.to_ne_bytes(), MapFlags::ANY)?;
    }

    println!("\nXDPDrop started");
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        std::thread::sleep(Duration::from_secs(1));
    }

    unsafe { xdp_detach(interface_id, mode) }
}
