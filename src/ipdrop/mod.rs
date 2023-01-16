use std::{
    net::Ipv4Addr,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use anyhow::{bail, Result};

mod xdpdrop {
    include!(concat!(env!("OUT_DIR"), "/xdpdrop.skel.rs"));
}

use libbpf_rs::MapFlags;
use xdpdrop::*;

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

unsafe fn attach_xdp_best_available(interface_id: i32, fd: i32) -> Result<u32> {
    for mode in [
        libbpf_sys::XDP_FLAGS_HW_MODE,
        libbpf_sys::XDP_FLAGS_DRV_MODE,
        libbpf_sys::XDP_FLAGS_SKB_MODE,
        libbpf_sys::XDP_FLAGS_UPDATE_IF_NOEXIST,
    ]
    .iter()
    {
        if xdp_attach(interface_id, fd, *mode).is_ok() {
            return Ok(*mode);
        }
        println!("Unable to load xdp program with mode {}", *mode);
    }

    bail!("Unable to attach xdp program to any interface")
}

unsafe fn xdp_attach(interface_id: i32, fd: i32, mode: u32) -> Result<()> {
    let err = libbpf_sys::bpf_xdp_attach(interface_id, fd, mode, std::ptr::null());
    if err != 0 {
        bail!("Unable to attach xdp program to interface")
    }
    Ok(())
}

unsafe fn xdp_detach(interface_id: i32, mode: u32) -> Result<()> {
    let err = libbpf_sys::bpf_xdp_detach(interface_id, mode, std::ptr::null());
    if err != 0 {
        bail!("Unable to detach xdp program from interface")
    }
    Ok(())
}

pub fn drop_ipv4_packets(target_ips: Vec<Ipv4Addr>) -> Result<()> {
    let mut skel_builder = XdpdropSkelBuilder::default();
    skel_builder.obj_builder.debug(true);
    let open_skel = skel_builder.open()?;

    // vars
    let interface_id = 2;

    bump_memlock_rlimit()?;

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
