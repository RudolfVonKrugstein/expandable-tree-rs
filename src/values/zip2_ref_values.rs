use super::{
    borrowed_value::{BorrowedValue, ItemOf},
    Zip3RefValues,
};

use crate::values::TreeValues;

pub struct Zip2RefValues<'a, A, B> {
    values: (&'a Vec<A>, &'a Vec<B>),
}

impl<'a, A, B> Zip2RefValues<'a, A, B> {
    fn get(&'a self, index: usize) -> (&'a A, &'a B) {
        (&self.values.0[index], &self.values.1[index])
    }

    pub fn from_vecs(values: (&'a Vec<A>, &'a Vec<B>)) -> Zip2RefValues<'a, A, B> {
        Zip2RefValues { values }
    }

    pub fn zip<N>(&'a self, other: &'a Vec<N>) -> Zip3RefValues<A, B, N> {
        Zip3RefValues::from_vecs((self.values.0, self.values.1, other))
    }

    pub fn split(self) -> (&'a Vec<A>, &'a Vec<B>) {
        (self.values.0, self.values.1)
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
