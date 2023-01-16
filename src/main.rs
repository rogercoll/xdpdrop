use anyhow::Result;

mod ipdrop;

fn main() -> Result<()> {
    ipdrop::drop_packets(8, 8, 8, 8)
}
