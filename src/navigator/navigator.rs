use super::neighbors::Neighbors;

#[derive(Debug)]
/** The navigator stores the structure of the tree.
 *
 * It knows which nodes are neighbors, parents and children
 * of a node and allows navigating to them.
 * With "navigation" we mean that the index in the flat Vector storing all the nodes
 * is returned.
*/
pub struct Navigator {
    neighbors: Vec<Neighbors<usize>>,
}

impl Navigator {
    /** Create a navigator given its internal data.
     * is and should only be used by the builder.
     */
    pub(crate) fn new(neighbors: Vec<Neighbors<usize>>) -> Navigator {
        Navigator { neighbors }
    }

    pub fn for_each_depth_first<F>(&self, mut f: F)
    where
        F: FnMut(usize, Vec<usize>),
    {
        (0..self.neighbors.len())
            .rev()
            .for_each(|i| f(i, self.children(i)))
    }

    /** Return the index of the parent (if any).
     *
     * # Arguments
     *
     * - index - The index to find the parent of.
     *
     * # Result
     *
     * The index of the parent or none if the node has no parent.
     */
    pub fn parent(&self, index: usize) -> Option<usize> {
        self.neighbors.get(index).and_then(|n| n.parent)
    }

    /** Return the index of the first child (if any).
     *
     * # Arguments
     *
     * - index - The index to find the first child of.
     *
     * # Result
     *
     * The index of the first child or none if the node has no first child.
     */
    pub fn first_child(&self, index: usize) -> Option<usize> {
        // the first child is the next node
        self.neighbors.get(index + 1).and_then(|i| {
            if i.parent.unwrap() != index {
                None
            } else {
                Some(index + 1)
            }
        })
    }

    /** Return the indices of the all children.
     *
     * # Arguments
     *
     * - index - The index to find the children of.
     *
     * # Result
     *
     * The indices of the childre in a `Vec`.
     */
    pub fn children(&self, index: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let mut opt_cindex = self.first_child(index);
        while let Some(cindex) = opt_cindex {
            res.push(cindex);
            opt_cindex = self.neighbors.get(cindex).unwrap().next_sibling;
        }
        res
    }

    /** Return the index of the the next sibling (if any).
     *
     * The next sibling is the next node that has the same parent
     * as the given node.
     *
     * # Arguments
     *
     * - index - The index to find the next sibling of.
     *
     * # Result
     *
     * The index of the next sibling or none if the node has no next sibling.
     */
    pub fn prev_sibling(&self, index: usize) -> Option<usize> {
        self.neighbors.get(index).and_then(|n| n.prev_sibling)
    }

    /** Return the index of the the prev sibling (if any).
     *
     * The prev sibling is the prev node that has the same parent
     * as the given node.
     *
     * # Arguments
     *
     * - index - The index to find the prev sibling of.
     *
     * # Result
     *
     * The index of the prev sibling or none if the node has no prev sibling.
     */
    pub fn next_sibling(&self, index: usize) -> Option<usize> {
        self.neighbors.get(index).and_then(|n| n.next_sibling)
    }
}
