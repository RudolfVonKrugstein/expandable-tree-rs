use crate::navigator::Neighbors;

pub trait TreeData: Copy {
    type Node;
    fn get(self, index: usize) -> Self::Node;
    fn get_neighbors(&self, index: usize) -> &Neighbors<usize>;
}
