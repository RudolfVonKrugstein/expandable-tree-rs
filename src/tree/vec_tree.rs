use crate::navigator::Navigator;

use super::subtree::subtree_impl::SubtreeImpl;
use super::{super::navigator::Neighbors, tree_data::TreeData, Tree};

pub struct VecTree<A> {
    pub nav: Navigator,
    pub values: Vec<A>,
}

impl<'a, A> TreeData for &'a VecTree<A> {
    type Node = &'a A;

    fn get(&self, index: usize) -> Self::Node {
        &self.values[index]
    }

    fn get_neighbors(&self, index: usize) -> &Neighbors<usize> {
        self.nav.get_neighbors(index)
    }
}

impl<A> Tree for VecTree<A> {
    fn root(&self) -> SubtreeImpl<VecTree<A>> {
        SubtreeImpl::new(self, 0)
    }
}
