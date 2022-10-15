use crate::navigator::Navigator;
use crate::tree::subtree::subtree_impl::SubtreeImpl;
use crate::tree::tree_data::TreeData;
use crate::Tree;

pub struct FlangedTree<TD, A>
where
    TD: TreeData,
{
    base: TD,
    data: Vec<A>,
}

impl<'a, TD, A> FlangedTree<TD, A>
where
    TD: TreeData,
{
    pub fn new(base: TD, data: Vec<A>) -> Self {
        FlangedTree { base, data }
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

    fn node_count(self) -> usize {
        self.base.node_count()
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
}
