use super::neighbors::Neighbors;

#[derive(Debug)]
pub struct Navigator {
    neighbors: Vec<Neighbors<usize>>,
}

impl Navigator {
    pub(crate) fn new(neighbors: Vec<Neighbors<usize>>) -> Navigator {
        Navigator {neighbors}
    }

    pub(crate) fn all_neighbors(&self) -> &Vec<Neighbors<usize>> {
        &self.neighbors
    }

    pub fn for_each_depth_first<F>(&self, mut f: F)
        where
            F: FnMut(usize, Vec<usize>) {
        (0..self.neighbors.len()).rev().for_each(
            |i| f(i, self.children(i))
        )
    }

    pub fn parent(&self, index: usize) -> Option<usize> {
        self.neighbors.get(index)
            .and_then(
                |n| n.parent
            )
    }

    pub fn first_child(&self, index: usize) -> Option<usize> {
        // the first child is the next node
        self.neighbors.get(index+1)
            .and_then(
                |i| if i.parent.unwrap() != index {
                    None
                } else {
                    Some(index+1)
                }
            )
    }

    pub fn children(&self, index: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let mut opt_cindex = self.first_child(index);
        while let Some(cindex) = opt_cindex {
            res.push(cindex);
            opt_cindex = self.neighbors.get(cindex).unwrap().next_sibling;
        }
        res
    }

    pub fn prev_sibling(&self, index: usize) -> Option<usize> {
        self.neighbors.get(index)
            .and_then(
                |n| n.prev_sibling
            )
    }

    pub fn next_sibling(&self, index: usize) -> Option<usize> {
        self.neighbors.get(index)
            .and_then(
                |n| n.next_sibling
            )
    }
}

