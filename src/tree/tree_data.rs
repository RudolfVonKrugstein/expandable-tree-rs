use super::super::navigator::Neighbors;

pub trait TreeData {
    type Node;
    fn get(&self, index: usize) -> Self::Node;
    fn get_neighbors(&self, index: usize) -> &Neighbors<usize>;
}
