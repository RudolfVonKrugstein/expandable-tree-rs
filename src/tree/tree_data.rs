use crate::navigator::{Navigator};

pub trait TreeData: Copy {
    type Node;
    fn node_count(self) -> usize;
    fn get(self, index: usize) -> Self::Node;
    fn get_nav(&self) -> &Navigator;
}
