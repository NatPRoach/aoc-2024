pub mod ordered_locations;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationId(usize);

impl LocationId {
    #[must_use]
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    #[must_use]
    pub fn abs_diff(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0)
    }
}

impl From<LocationId> for usize {
    fn from(value: LocationId) -> usize {
        value.0
    }
}

impl From<&LocationId> for usize {
    fn from(value: &LocationId) -> usize {
        value.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    id: LocationId,
}

impl Location {
    #[must_use]
    pub fn new(id: LocationId) -> Location {
        Self { id }
    }

    pub(crate) fn id(&self) -> &LocationId {
        &self.id
    }
}
