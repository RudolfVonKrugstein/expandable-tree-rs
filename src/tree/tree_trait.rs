use crate::{FlangedTree, Subtree};

use super::tree_data::TreeData;
use super::MappedTree;

pub trait Tree<'a>: Sized
where
    Self: 'a,
    &'a Self: TreeData<Node = Self::Node>,
{
    type Node;
    type SubtreeType: Subtree<Node = Self::Node>;

    fn root(&'a self) -> Self::SubtreeType;

    fn map<B, M>(&'a self, m: M) -> MappedTree<Self::Node, B, M, &'a Self>
    where
        M: Fn(usize, Self::Node) -> B,
    {
        MappedTree::new(self, m)
    }

    fn flange<B>(&'a self, data: &'a Vec<B>) -> FlangedTree<&'a Self, B> {
        FlangedTree::new(self, data)
    }
}

// impl<A: Sized, Node> Tree for A
// where
//     for<'a> &'a A: TreeData<Node = &'a Node>,
//     for<'a> Node: 'a,
// {
//     fn root(&self) -> SubtreeImpl<A> {
//         SubtreeImpl::new(self, 0)
//     }
// }
