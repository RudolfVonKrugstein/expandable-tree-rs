use super::subtree::subtree_impl::SubtreeImpl;
use super::tree_data::TreeData;

pub trait Tree: Sized
where
    for<'a> &'a Self: TreeData,
{
    fn root(&self) -> SubtreeImpl<Self>;
}
