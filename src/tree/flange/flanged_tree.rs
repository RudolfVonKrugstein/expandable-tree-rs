use crate::navigator::Navigator;
use crate::tree::subtree::subtree_impl::SubtreeImpl;
use crate::tree::tree_data::TreeData;
use crate::{Subtree, Tree};

pub struct FlangedTree<TD, A>
where
    TD: TreeData,
{
    base: TD,
    data: Vec<A>,
}

impl<TD, A> FlangedTree<TD, A>
where
    TD: TreeData,
{
    pub fn new(base: TD, data: Vec<A>) -> Self {
        FlangedTree { base, data }
    }

    pub fn replace_map_flange<'a, B, F>(&'a self, mapf: F) -> FlangedTree<TD, B>
    where
        F: Fn(<FlangedTree<TD, A> as Tree<'a>>::Node) -> B,
    {
        // Uninitialized vector
        let mut res = Vec::with_capacity(self.node_count());

        self.for_each(|s| {
            res.push(mapf(s.value()));
        });
        FlangedTree::new(self.base, res)
    }

    pub fn get_flange(&self, index: usize) -> &A {
        &self.data[index]
    }

    pub fn get_flange_mut(&mut self, index: usize) -> &mut A {
        &mut self.data[index]
    }
}

impl<'a, TD, Node, A> TreeData for &'a FlangedTree<TD, A>
where
    TD: TreeData<Node = Node>,
{
    type Node = (Node, &'a A);

    fn get(self, index: usize) -> Self::Node {
        (self.base.get(index), &self.data[index])
    }

    fn count(self) -> usize {
        self.base.count()
    }

    fn get_nav(&self) -> &Navigator {
        self.base.get_nav()
    }
}

impl<'a, TD, A> Tree<'a> for FlangedTree<TD, A>
where
    A: 'a,
    TD: TreeData + 'a,
{
    type Node = (TD::Node, &'a A);
    type SubtreeType = SubtreeImpl<&'a FlangedTree<TD, A>>;

    fn at_pos(&'a self, index: usize) -> Self::SubtreeType {
        SubtreeImpl::new(self, index)
    }
    fn get_nav(&self) -> &Navigator {
        self.base.get_nav()
    }
}
