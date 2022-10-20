use crate::navigator::Navigator;
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

    fn at_pos(&'a self, index: usize) -> Self::SubtreeType;

    fn get_nav(&self) -> &Navigator;

    fn root(&'a self) -> Self::SubtreeType {
        self.at_pos(0)
    }

    fn node_count(&'a self) -> usize {
        self.count()
    }

    fn map<B, M>(&'a self, m: M) -> MappedTree<Self::Node, B, M, &'a Self>
    where
        M: Fn(usize, Self::Node) -> B,
    {
        MappedTree::new(self, m)
    }

    fn flange<B>(&'a self, data: Vec<B>) -> FlangedTree<&'a Self, B> {
        FlangedTree::new(self, data)
    }

    fn flange_map<B,F>(&'a self, mapf: F) -> FlangedTree<&'a Self, B>
    where
        B: 'a,
        B: Clone,
        F: Fn(Self::Node) -> B {
        let mut res = Vec::with_capacity(self.node_count());

        for index in 0..self.count() {
            let new_val = mapf(self.get(index));
            res.push(
                new_val
            );
        }
        FlangedTree::new(self, res)
    }

    fn for_each<F>(&'a self, mut f: F)
    where
        F: FnMut(Self::SubtreeType),
    {
        for i in 0..self.count() {
            f(self.at_pos(i))
        }
    }

    fn depth_first_flange<B, F>(&'a self, mapf: F) -> FlangedTree<&'a Self, B>
    where
        B: 'a,
        B: Default,
        B: Clone,
        F: Fn(Self::Node, Vec<&B>) -> B,
    {
        // Uninitialized vector
        let mut res = vec![B::default(); self.node_count()];

        self.get_nav().for_each_depth_first(|i, childs| {
            let new_val = mapf(self.get(i), childs.iter().map(|&c| &res[c]).collect());
            res[i] = new_val;
        });
        FlangedTree::new(self, res)
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
