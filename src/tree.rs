use crate::navigator::Navigator;
use crate::subtree::Subtree;
use crate::values::{TreeValues, VecValues, Zip2Values, Zip3Values};

#[derive(Debug)]
pub struct FlatTree<TV>
where
    TV: TreeValues,
{
    pub(crate) nav: Navigator,
    pub(crate) values: TV,
}

impl<TV: TreeValues> FlatTree<TV> {
    pub fn root(&self) -> Subtree<TV> {
        Subtree::new(&self.values, &self.nav, 0)
    }
}

impl<A> FlatTree<VecValues<A>> {
    // Destructive!
    pub fn expand<B>(self: Self, new_values: Vec<B>) -> FlatTree<Zip2Values<A, B>> {
        FlatTree {
            nav: self.nav,
            values: self.values.zip(new_values),
        }
    }
}

impl<A, B> FlatTree<Zip2Values<A, B>> {
    // Destructive!
    pub fn expand<N>(self: Self, new_values: Vec<N>) -> FlatTree<Zip3Values<A, B, N>> {
        FlatTree {
            nav: self.nav,
            values: self.values.zip(new_values),
        }
    }
}
