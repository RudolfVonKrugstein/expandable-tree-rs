use super::borrowed_value::{BorrowedValue, ItemOf};

use crate::values::TreeValues;

pub struct Zip2RefValues<'a, A, B> {
    values: (&'a Vec<A>, &'a Vec<B>),
}

impl<'a, A, B> Zip2RefValues<'a, A, B> {
    fn get(&'a self, index: usize) -> (&'a A, &'a B) {
        (&self.values.0[index], &self.values.1[index])
    }

    pub fn from_vecs(first: &'a Vec<A>, second: &'a Vec<B>) -> Zip2RefValues<'a, A, B> {
        Zip2RefValues {
            values: (first, second),
        }
    }
}

impl<'a, 'b, A, B> BorrowedValue<'a> for Zip2RefValues<'b, A, B> {
    type Item = (&'a A, &'a B);
}

impl<'a, A, B> TreeValues for Zip2RefValues<'a, A, B> {
    fn get(&self, index: usize) -> ItemOf<'_, Self> {
        self.get(index)
    }
}
