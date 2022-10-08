use super::borrowed_value::{BorrowedValue, ItemOf};
use crate::values::TreeValues;

pub struct Zip3Values<A, B, C> {
    values: (Vec<A>, Vec<B>, Vec<C>),
}

impl<A, B, C> Zip3Values<A, B, C> {
    fn get(&self, index: usize) -> (&A, &B, &C) {
        (
            &self.values.0[index],
            &self.values.1[index],
            &self.values.2[index],
        )
    }

    pub fn from_vecs(old: (Vec<A>, Vec<B>), new: Vec<C>) -> Self {
        Zip3Values {
            values: (old.0, old.1, new),
        }
    }
}

impl<'a, A, B, C> BorrowedValue<'a> for Zip3Values<A, B, C> {
    type Item = (&'a A, &'a B, &'a C);
}

impl<A, B, C> TreeValues for Zip3Values<A, B, C> {
    fn get(&self, index: usize) -> ItemOf<'_, Self> {
        self.get(index)
    }
}
