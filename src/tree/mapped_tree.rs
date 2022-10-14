use super::subtree::subtree_impl::SubtreeImpl;
use crate::{navigator::Neighbors, Tree};

use super::tree_data::TreeData;

pub struct MappedTree<A, B, M, TD>
where
    M: Fn(usize, A) -> B,
    TD: TreeData<Node = A>,
{
    mapf: M,
    base: TD,
}

impl<A, B, M, TD> MappedTree<A, B, M, TD>
where
    M: Fn(usize, A) -> B,
    TD: TreeData<Node = A>,
{
    pub fn new(base: TD, mapf: M) -> Self {
        MappedTree { mapf, base }
    }
}

impl<'a, A, B, M, TD> TreeData for &'a MappedTree<A, B, M, TD>
where
    M: Fn(usize, A) -> B,
    TD: TreeData<Node = A>,
{
    type Node = B;

    fn get(&self, index: usize) -> Self::Node {
        (self.mapf)(index, self.base.get(index))
    }

    fn get_neighbors(&self, index: usize) -> &Neighbors<usize> {
        self.base.get_neighbors(index)
    }
}

impl<A, B, M, TD> Tree for MappedTree<A, B, M, TD>
where
    M: Fn(usize, A) -> B,
    TD: TreeData<Node = A>,
{
    fn root<'a>(&'a self) -> SubtreeImpl<'a, MappedTree<A, B, M, TD>>
    where
        B: 'a,
        &'a Self: TreeData,
    {
        SubtreeImpl::new(self, 0)
    }
}

// impl<'a, T: Tree, Node: MappedTreeNode> TreeData for &'a MappedTree<T, Node>
// where
//     for<'b> &'b T: TreeData,
// {
//     type Node = Node;
//     fn get(&self, index: usize) -> Self::Node {
//         Node::create((&self.base).get(index), &self.data[index])
//     }

//     fn get_neighbors(&self, index: usize) -> &Neighbors<usize> {
//         (&self.base).get_neighbors(index)
//     }
// }
