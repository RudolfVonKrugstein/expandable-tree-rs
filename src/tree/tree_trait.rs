use super::subtree::subtree_impl::SubtreeImpl;
use super::tree_data::TreeData;

pub trait Tree: Sized {
    fn root<'a>(&'a self) -> SubtreeImpl<'a, Self>
    where
        &'a Self: TreeData;
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
