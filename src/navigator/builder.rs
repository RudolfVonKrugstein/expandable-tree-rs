use super::navigator::Navigator;
use super::neighbors::Neighbors;

pub struct Builder {
    current_depth: usize,
    parents_stack: Vec<usize>,
    last_sibling: Option<usize>,

    cur_neighbors: Vec<Neighbors<usize>>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            cur_neighbors: Vec::new(),
            current_depth: 0,
            parents_stack: Vec::new(),
            last_sibling: None,
        }
    }

    pub fn with_capacity(c: usize) -> Builder {
        Builder {
            cur_neighbors: Vec::with_capacity(c),
            current_depth: 0,
            parents_stack: Vec::new(),
            last_sibling: None,
        }
    }

    // Building
    pub fn start_element(&mut self) -> usize {
        let my_index = self.cur_neighbors.len();
        self.cur_neighbors.push(
            Neighbors {
                me: Some(my_index),
                parent: self.parents_stack.last().cloned(),
                next_sibling: None,
                prev_sibling: self.last_sibling,
            }
        );
        // Update last sibling
        if let Some(ls) = self.last_sibling {
            self.cur_neighbors[ls].next_sibling = Some(my_index);
        }
        // Update state
        self.parents_stack.push(my_index);
        self.last_sibling = None;
        my_index

    }
    pub fn end_element(&mut self) -> usize {
        self.last_sibling = self.parents_stack.pop();
        self.last_sibling.unwrap()
    }
    pub fn start_end_element(&mut self) -> usize {
        let my_index = self.cur_neighbors.len();
        self.cur_neighbors.push(Neighbors {
            me: Some(my_index),
            parent: self.parents_stack.last().cloned(),
            next_sibling: None,
            prev_sibling: self.last_sibling,
        });
        // Update last sibling
        if let Some(ls) = self.last_sibling {
            self.cur_neighbors[ls].next_sibling = Some(my_index);
        }
        // Update state
        self.last_sibling = Some(my_index);
        my_index
    }

    pub fn build(self) -> Navigator {
        Navigator::new(self.cur_neighbors)
    }
}
