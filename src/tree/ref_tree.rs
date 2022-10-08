use super::subtree::Subtree;
use crate::{
    navigator::Navigator,
    values::{TreeValues, Zip2RefValues, Zip3RefValues},
};

#[derive(Debug)]
pub struct RefFlatTree<'a, TV>
where
    TV: TreeValues,
{
    pub(crate) nav: &'a Navigator,
    pub(crate) values: TV,
}

impl<'a, TV: TreeValues> RefFlatTree<'a, TV> {
    pub fn root(&self) -> Subtree<TV> {
        Subtree::new(&self.values, &self.nav, 0)
    }
}

// A ref tree cannot be flanged!

impl<'a, A, B> RefFlatTree<'a, Zip2RefValues<'a, A, B>> {
    // Destructive!
    pub fn ref_flange<N>(
        &'a self,
        new_values: &'a Vec<N>,
    ) -> RefFlatTree<'a, Zip3RefValues<'a, A, B, N>> {
        RefFlatTree {
            nav: self.nav,
            values: self.values.zip(new_values),
        }
    }
}
