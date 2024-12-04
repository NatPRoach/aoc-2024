use crate::locations::ordered_locations::OrderedLocations;
use crate::locations::{Location, LocationId};
use crate::AdventOfCodeError;
use std::io::BufRead;
use std::io::Read;
use std::str::FromStr;

const BUFFER_SIZE: usize = 1024;

pub struct Reader<R: Read + BufRead> {
    inner: R,
    buffer: String,
}

impl<R: Read + BufRead> Reader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buffer: String::with_capacity(BUFFER_SIZE),
        }
    }

    fn parse_pair(s: &str) -> Result<Option<(LocationId, LocationId)>, AdventOfCodeError> {
        let mut split_line = s.split_whitespace();
        let column1 = LocationId::new(split_line.next().map_or(
            Err(AdventOfCodeError::LocationPairParseError),
            |s| {
                Ok(usize::from_str(s)?)
            },
        )?);
        let column2 = LocationId::new(split_line.next().map_or(
            Err(AdventOfCodeError::LocationPairParseError),
            |s| {
                Ok(usize::from_str(s)?)
            },
        )?);
        Ok(Some((column1, column2)))
    }

    #[must_use]
    pub fn read_pair(&mut self) -> Result<Option<(LocationId, LocationId)>, AdventOfCodeError> {
        self.buffer.clear();
        self.inner.read_line(&mut self.buffer)?;
        if self.buffer.is_empty() {
            return Ok(None);
        }
        Self::parse_pair(&self.buffer)
    }

    #[must_use]
    pub fn read_pair_unchecked(&mut self) -> Option<(LocationId, LocationId)> {
        self.read_pair().unwrap()
    }

    pub fn drain_into_ordered_locations(self) -> (OrderedLocations, OrderedLocations) {
        let mut list1 = Vec::with_capacity(1000);
        let mut list2 = Vec::with_capacity(1000);
        for (element1, element2) in self {
            list1.push(Location::new(element1));
            list2.push(Location::new(element2));
        }
        list1.sort();
        list2.sort();

        (OrderedLocations::new(list1), OrderedLocations::new(list2))
    }
}

impl<R: Read + BufRead> Iterator for Reader<R> {
    type Item = (LocationId, LocationId);
    fn next(&mut self) -> Option<Self::Item> {
        self.read_pair_unchecked()
    }
}
