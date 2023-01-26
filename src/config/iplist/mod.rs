use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::{net::Ipv4Addr, str::FromStr};

pub(crate) fn from_file(file_path: &str) -> Result<Vec<Ipv4Addr>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents
        .split('\n')
        .filter(|line| !line.contains("#"))
        .flat_map(Ipv4Addr::from_str)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_ips_from_file() {
        let sample_list = "
# ------------------------------------[UPDATE]--------------------------------------
# Title: The Block List Project - Tracking List
# Expires: 1 day
# Homepage: https://blocklist.site
# Help: https://github.com/blocklistproject/lists/wiki/
# License: https://unlicense.org
# Total number of network filters: 15057
# ------------------------------------[SUPPORT]-------------------------------------
# You can support by:
# - reporting false positives
# - making a donation: https://paypal.me/blocklistproject
# -------------------------------------[INFO]---------------------------------------
#
# tracking list
# ------------------------------------[FILTERS]-------------------------------------
212.150.34.116
213.239.214.240
63.214.247.19
66.226.74.5
81.19.88.106
81.95.145.240
91.215.100.37
91.215.100.38
91.215.101.35
91.215.101.36
91.215.103.65
1.1.1.107";

        let mut tmpfile = NamedTempFile::new().unwrap();
        tmpfile.write_all(sample_list.as_bytes()).unwrap();

        let list = from_file(tmpfile.path().to_str().unwrap()).unwrap();
        assert_eq!(12, list.len());
        assert_eq!(Ipv4Addr::new(1, 1, 1, 107), list[11]);
    }
}
