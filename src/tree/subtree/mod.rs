pub mod subtree_impl;

pub trait Subtree: Sized {
    type Node;

    fn get_pos(&self) -> usize;
    fn value(&self) -> Self::Node;
    fn parent(&self) -> Option<Self>;
    fn children(&self) -> Vec<Self>;
    fn child_values(&self) -> Vec<Self::Node>;
    fn first_child(&self) -> Option<Self>;
    fn prev_sibling(&self) -> Option<Self>;
    fn next_sibling(&self) -> Option<Self>;
}
