use anyhow::Result;
use std::fs::File;
use std::io::Read;

// parses line that defines a dns to filter, skips lines with comments
// ip + white_space + domain name
// e.g: 0.0.0.0 amazon.es
// TODO: verify correct format of ip and dns
fn parse_filter(line: &str) -> Option<String> {
    if line.contains("#") {
        return None;
    }
    Some(line.split_once(" ")?.1.to_string())
}

pub(crate) fn from_file(file_path: &str) -> Result<Vec<String>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents.split('\n').flat_map(parse_filter).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_dns_fromfile() {
        let sample_list = "# this is a comment
# another comment
0.0.0.0 amazon.es
0.0.0.0 sport.es";

        let mut tmpfile = NamedTempFile::new().unwrap();
        tmpfile.write_all(sample_list.as_bytes()).unwrap();

        let list = from_file(tmpfile.path().to_str().unwrap()).unwrap();
        assert_eq!(2, list.len());
        assert_eq!("amazon.es".to_string(), list[0]);
    }
}
