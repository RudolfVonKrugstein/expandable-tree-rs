use super::Subtree;

use super::super::tree_data::TreeData;

pub struct SubtreeImpl<TD>
where
    TD: TreeData,
{
    data: TD,
    pos: usize,
}

impl<'a, TD> SubtreeImpl<TD>
where
    TD: TreeData,
{
    pub fn new(data: TD, pos: usize) -> Self {
        SubtreeImpl { data, pos }
    }

    fn moved(&self, new_pos: usize) -> Self {
        SubtreeImpl {
            data: self.data,
            pos: new_pos,
        }
    }

    fn children_indexes(&self) -> Vec<usize> {
        let mut res = Vec::new();
        let mut oc = self.data.get_neighbors(self.pos).first_child;
        while let Some(c) = oc {
            // Store the child
            res.push(c);
            // Advance to the next child
            oc = self.data.get_neighbors(c).next_sibling;
        }
        res
    }
}

impl<TD> Subtree for SubtreeImpl<TD>
where
    TD: TreeData,
{
    type Node = TD::Node;

    fn value(&self) -> TD::Node {
        self.data.get(self.pos)
    }
    fn parent(&self) -> Option<Self> {
        self.data
            .get_neighbors(self.pos)
            .parent
            .map(|i| self.moved(i))
    }
    fn children(&self) -> Vec<Self> {
        self.children_indexes()
            .iter()
            .map(|&i| self.moved(i))
            .collect()
    }
    fn child_values(&self) -> Vec<Self::Node> {
        self.children_indexes()
            .iter()
            .map(|&i| self.data.get(i))
            .collect()
    }
    fn first_child(&self) -> Option<Self> {
        self.data
            .get_neighbors(self.pos)
            .first_child
            .map(|i| self.moved(i))
    }
    fn prev_sibling(&self) -> Option<Self> {
        self.data
            .get_neighbors(self.pos)
            .prev_sibling
            .map(|i| self.moved(i))
    }
    fn next_sibling(&self) -> Option<Self> {
        self.data
            .get_neighbors(self.pos)
            .next_sibling
            .map(|i| self.moved(i))
    }
}
