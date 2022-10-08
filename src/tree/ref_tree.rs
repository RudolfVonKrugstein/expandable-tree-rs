use super::subtree::Subtree;
use crate::{navigator::Navigator, values::TreeValues};

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
