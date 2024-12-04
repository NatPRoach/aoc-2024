use std::collections::HashMap;

use crate::locations::Location;

pub struct OrderedLocations {
    inner: Vec<Location>,
}

impl OrderedLocations {
    #[must_use]
    pub fn new(inner: Vec<Location>) -> Self {
        Self { inner }
    }

    #[must_use]
    pub fn sorted_slice(&self) -> &[Location] {
        &self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

}

impl<I: IntoIterator<Item = Location>> From<I> for OrderedLocations {
    fn from(value: I) -> Self {
        let inner = value.into_iter().collect();
        Self::new(inner)
    }
}
