use crate::navigator::Navigator;

use super::subtree::subtree_impl::SubtreeImpl;
use super::{tree_data::TreeData, Tree};

pub struct VecTree<A> {
    pub nav: Navigator,
    pub values: Vec<A>,
}

impl<'a, A> TreeData for &'a VecTree<A> {
    type Node = &'a A;

    fn get(self, index: usize) -> Self::Node {
        &self.values[index]
    }

    fn count(self) -> usize {
        self.values.len()
    }

    fn get_nav(&self) -> &Navigator {
        &self.nav
    }
}

impl<'a, A> Tree<'a> for VecTree<A>
where
    A: 'a,
{
    type Node = &'a A;
    type SubtreeType = SubtreeImpl<&'a VecTree<A>>;

    fn at_pos(&'a self, index: usize) -> Self::SubtreeType {
        SubtreeImpl::new(self, index)
    }
}
