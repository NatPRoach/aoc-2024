use crate::locations::ordered_locations::OrderedLocations;

pub struct LocationListMerge {
    list1_order: OrderedLocations,
    list2_order: OrderedLocations,
}

impl LocationListMerge {
    #[must_use]
    pub fn new(list1_order: OrderedLocations, list2_order: OrderedLocations) -> Self {
        Self {
            list1_order,
            list2_order,
        }
    }

    #[must_use]
    pub fn calculate_total_distance(self) -> usize {
        self.list1_order
            .sorted_slice()
            .iter()
            .zip(self.list2_order.sorted_slice().iter())
            .map(|(list1_location, list2_location)| list1_location.id.abs_diff(&list2_location.id))
            .sum()
    }

}
