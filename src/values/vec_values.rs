use super::borrowed_value::{BorrowedValue, ItemOf};
use super::TreeValues;
use crate::values::zip2_values::Zip2Values;

pub struct VecValues<A> {
    values: Vec<A>,
}

impl<A> VecValues<A> {
    fn get(&self, index: usize) -> &A {
        &self.values[index]
    }

    pub fn from_vec(values: Vec<A>) -> VecValues<A> {
        VecValues { values }
    }

    pub fn zip<B>(self, other: Vec<B>) -> Zip2Values<A, B> {
        Zip2Values::from_vecs(self.values, other)
    }
}

impl<'a, A> BorrowedValue<'a> for VecValues<A> {
    type Item = &'a A;
}

impl<A> TreeValues for VecValues<A> {
    fn get(&self, index: usize) -> ItemOf<'_, Self> {
        self.get(index)
    }
}
