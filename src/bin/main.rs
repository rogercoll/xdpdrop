use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs)]
/// xdpdrop CLI config
struct CLI {
    /// dns blocklist file
    #[argh(option)]
    dns_list: Option<String>,
    /// ip blocklist file
    #[argh(option)]
    ip_list: Option<String>,
    /// device interface name to attach the xdp program into
    #[argh(option, short = 'i')]
    interface: String,
}

fn main() -> Result<()> {
    let cli_conf: CLI = argh::from_env();

    xdpdrop::drop(&cli_conf.interface, cli_conf.ip_list, cli_conf.dns_list)
}
