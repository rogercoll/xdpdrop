use anyhow::{bail, Result};

mod xdpdrop {
    include!(concat!(env!("OUT_DIR"), "/xdpdrop.skel.rs"));
}

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

unsafe fn attach_xdp_best_available(interface_id: i32, fd: i32) -> Result<()> {
    for mode in [
        libbpf_sys::XDP_FLAGS_HW_MODE,
        libbpf_sys::XDP_FLAGS_DRV_MODE,
        libbpf_sys::XDP_FLAGS_SKB_MODE,
        libbpf_sys::XDP_FLAGS_UPDATE_IF_NOEXIST,
    ]
    .iter()
    {
        if xdp_attach(interface_id, fd, *mode).is_ok() {
            return Ok(());
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

fn main() -> Result<()> {
    let mut skel_builder = XdpdropSkelBuilder::default();
    let mut open_skel = skel_builder.open()?;

    bump_memlock_rlimit()?;

    let mut skel = open_skel.load()?;

    unsafe { attach_xdp_best_available(2, skel.progs_mut().xdp_drop_prog().fd())? }
    // let link = skel.progs_mut().xdp_drop_prog().attach_xdp(2)?;
    loop {}
    Ok(())
}
