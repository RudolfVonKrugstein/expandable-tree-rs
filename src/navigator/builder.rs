use super::navigator_impl::Navigator;
use super::neighbors::Neighbors;

/** Create a FlatTree using simple commands
A flat tree itself is immutable, so we use the builder pattern.
## Example

```
use flange_flat_tree::navigator::Builder;

// A navigator just stores neigbors, so we provide no values
let mut b = Builder::default();
let root = b.start_element();
let child = b.start_end_element();

let nav = b.build();

assert_eq!(nav.children(root), [child]);
assert_eq!(nav.parent(child), Some(root));
```
 */
pub struct Builder {
    // current_depth: usize,

    // List of the parents at the different levels (so the last one is always
    // the current parent).
    parents_stack: Vec<usize>,
    // The last sibling for our current position (if any).
    last_sibling: Option<usize>,

    // Internal data for the final tree.
    cur_neighbors: Vec<Neighbors<usize>>,
}

impl Default for Builder {
    /// Create a new builder
    fn default() -> Builder {
        Builder {
            cur_neighbors: Vec::new(),
            // current_depth: 0,
            parents_stack: Vec::new(),
            last_sibling: None,
        }
    }
}

impl Builder {
    /// Create a new builder and pre-allocate memory
    pub fn with_capacity(c: usize) -> Builder {
        Builder {
            cur_neighbors: Vec::with_capacity(c),
            // current_depth: 0,
            parents_stack: Vec::new(),
            last_sibling: None,
        }
    }

    /** Start a new element.
     *
     * This element will be the child of the last element
     * that was not finished with `end_element`.
     */
    pub fn start_element(&mut self) -> usize {
        let my_index = self.cur_neighbors.len();
        self.cur_neighbors.push(Neighbors {
            me: Some(my_index),
            parent: self.parents_stack.last().cloned(),
            first_child: None,
            next_sibling: None,
            prev_sibling: self.last_sibling,
        });
        // Update last sibling
        if let Some(ls) = self.last_sibling {
            self.cur_neighbors[ls].next_sibling = Some(my_index);
        }
        // Update parent
        if let Some(&parent_index) = self.parents_stack.last() {
            if self.cur_neighbors[parent_index].first_child.is_none() {
                self.cur_neighbors[parent_index].first_child = Some(my_index);
            }
        }
        // Update state
        self.parents_stack.push(my_index);
        self.last_sibling = None;
        my_index
    }

    /** End and element.
     *
     * No more children will be added to this element.
     * The next element would be a sibling of this element.
     */
    pub fn end_element(&mut self) -> usize {
        self.last_sibling = self.parents_stack.pop();
        self.last_sibling.unwrap()
    }

    /** Start and imidiatly end an element.
     *
     * This is the same as calling `start_element` and `end_element`
     * after each other.
     */
    pub fn start_end_element(&mut self) -> usize {
        let my_index = self.cur_neighbors.len();
        self.cur_neighbors.push(Neighbors {
            me: Some(my_index),
            first_child: None,
            parent: self.parents_stack.last().cloned(),
            next_sibling: None,
            prev_sibling: self.last_sibling,
        });
        // Update last sibling
        if let Some(ls) = self.last_sibling {
            self.cur_neighbors[ls].next_sibling = Some(my_index);
        }
        // Update parent
        if let Some(&parent_index) = self.parents_stack.last() {
            if self.cur_neighbors[parent_index].first_child.is_none() {
                self.cur_neighbors[parent_index].first_child = Some(my_index);
            }
        }
        // Update state
        self.last_sibling = Some(my_index);
        my_index
    }

    /** Finish the building and create the `Navigator`.
     *
     * The navigator will be created and the buidler destructed.
     */
    pub fn build(self) -> Navigator {
        Navigator::new(self.cur_neighbors)
    }
}
