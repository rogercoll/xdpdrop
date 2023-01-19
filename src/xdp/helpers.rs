use anyhow::{bail, Result};

pub(crate) unsafe fn attach_xdp_best_available(interface_id: i32, fd: i32) -> Result<u32> {
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

pub(crate) unsafe fn xdp_detach(interface_id: i32, mode: u32) -> Result<()> {
    let err = libbpf_sys::bpf_xdp_detach(interface_id, mode, std::ptr::null());
    if err != 0 {
        bail!("Unable to detach xdp program from interface")
    }
    Ok(())
}
