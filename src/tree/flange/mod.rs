use crate::navigator::Neighbors;
use crate::tree::subtree::subtree_impl::SubtreeImpl;
use crate::tree::tree_data::TreeData;
use crate::Tree;

pub struct FlangedTree<'a, TD, A>
where
    TD: TreeData,
{
    base: TD,
    data: &'a Vec<A>,
}

impl<'a, TD, A> FlangedTree<'a, TD, A>
where
    TD: TreeData,
{
    pub fn new(base: TD, data: &'a Vec<A>) -> Self {
        FlangedTree { base, data }
    }
}

impl<'a, TD, Node, A> TreeData for &'a FlangedTree<'a, TD, A>
where
    TD: TreeData<Node = Node>,
{
    type Node = (Node, &'a A);

    fn get(self, index: usize) -> Self::Node {
        (self.base.get(index), &self.data[index])
    }

    fn get_neighbors(&self, index: usize) -> &Neighbors<usize> {
        self.base.get_neighbors(index)
    }
}

impl<'a, TD, A> Tree<'a> for FlangedTree<'a, TD, A>
where
    TD: TreeData + 'a,
{
    type Node = (TD::Node, &'a A);
    type SubtreeType = SubtreeImpl<&'a FlangedTree<'a, TD, A>>;

    fn root(&'a self) -> Self::SubtreeType {
        SubtreeImpl::new(self, 0)
    }
}
