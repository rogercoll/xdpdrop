use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs)]
/// xdpdrop CLI config
struct CLI {
    /// configuration file
    #[argh(option, short = 'f')]
    file: String,
}

fn main() -> Result<()> {
    let cli_conf: CLI = argh::from_env();

    xdpdrop::drop(&cli_conf.file)
}
