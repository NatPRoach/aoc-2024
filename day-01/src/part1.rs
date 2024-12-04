use aoc_lib::io::PairedLocationIdTsvReader;
use aoc_lib::locations::location_list_merge::LocationListMerge;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[tracing::instrument]
pub fn process<P: AsRef<Path> + Debug>(input: P) -> anyhow::Result<String> {
    let buf_reader = BufReader::with_capacity(1000, File::open(input).unwrap());
    let format_reader = PairedLocationIdTsvReader::new(buf_reader);
    let (order1, order2) = format_reader.drain_into_ordered_locations();
    let list_merge = LocationListMerge::new(order1, order2);
    let dist = list_merge.calculate_total_distance();
    Ok(format!("{dist}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::Path;
    use tempdir::TempDir;

    fn write_to_file<P: AsRef<Path>>(data: &str, file: P) {
        let mut file = std::fs::File::create(file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        let dir = TempDir::new("test")?;
        let file = dir.path().join("input1.txt");
        write_to_file(input, &file);

        assert_eq!("", process(&file)?);
        Ok(())
    }
}
