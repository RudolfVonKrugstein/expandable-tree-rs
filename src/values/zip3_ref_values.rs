use super::borrowed_value::{BorrowedValue, ItemOf};

use crate::values::TreeValues;

pub struct Zip3RefValues<'a, A, B, C> {
    values: (&'a Vec<A>, &'a Vec<B>, &'a Vec<C>),
}

impl<'a, A, B, C> Zip3RefValues<'a, A, B, C> {
    fn get(&'a self, index: usize) -> (&'a A, &'a B, &'a C) {
        (
            &self.values.0[index],
            &self.values.1[index],
            &self.values.2[index],
        )
    }

    pub fn from_vecs(values: (&'a Vec<A>, &'a Vec<B>, &'a Vec<C>)) -> Zip3RefValues<'a, A, B, C> {
        Zip3RefValues { values }
    }
}

impl<'a, 'b, A, B, C> BorrowedValue<'a> for Zip3RefValues<'b, A, B, C> {
    type Item = (&'a A, &'a B, &'a C);
}

impl<'a, A, B, C> TreeValues for Zip3RefValues<'a, A, B, C> {
    fn get(&self, index: usize) -> ItemOf<'_, Self> {
        self.get(index)
    }
}
