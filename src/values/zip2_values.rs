use super::{
    borrowed_value::{BorrowedValue, ItemOf},
    VecValues, Zip3Values,
};
use crate::values::TreeValues;

pub struct Zip2Values<A, B> {
    values: (Vec<A>, Vec<B>),
}

impl<A, B> Zip2Values<A, B> {
    fn get(&self, index: usize) -> (&A, &B) {
        (&self.values.0[index], &self.values.1[index])
    }

    pub fn from_vecs(first: Vec<A>, second: Vec<B>) -> Zip2Values<A, B> {
        Zip2Values {
            values: (first, second),
        }
    }

    pub fn zip<N>(self, other: Vec<N>) -> Zip3Values<A, B, N> {
        Zip3Values::from_vecs(self.values, other)
    }

    pub fn split(self) -> (VecValues<A>, Vec<B>) {
        (VecValues::from_vec(self.values.0), self.values.1)
    }
}

impl<'a, A, B> BorrowedValue<'a> for Zip2Values<A, B> {
    type Item = (&'a A, &'a B);
}

impl<A, B> TreeValues for Zip2Values<A, B> {
    fn get(&self, index: usize) -> ItemOf<'_, Self> {
        self.get(index)
    }
}
