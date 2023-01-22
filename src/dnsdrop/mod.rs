use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use anyhow::Result;

mod xdpdrop {
    include!(concat!(env!("OUT_DIR"), "/dnsdrop.skel.rs"));
}

use libbpf_rs::MapFlags;
use xdpdrop::*;

use crate::xdp::helpers::{attach_xdp_best_available, xdp_detach};

pub fn drop_dns(target_dns: Vec<String>) -> Result<()> {
    let mut skel_builder = DnsdropSkelBuilder::default();
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
        unsafe { attach_xdp_best_available(interface_id, skel.progs_mut().xdp_dns_drop().fd())? };

    let vals: u32 = 0;
    for dns in target_dns.iter() {
        let mut dns_array = [0u8; 128];
        dns_array[..dns.len()].copy_from_slice(dns.as_bytes());
        skel.maps_mut()
            .recordmap()
            .update(&dns_array, &vals.to_ne_bytes(), MapFlags::ANY)?;
    }

    println!("\nXDPDrop started");
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        std::thread::sleep(Duration::from_secs(1));
    }

    unsafe { xdp_detach(interface_id, mode) }
}
