use crate::VecTree;
/**

# Example:

```
use flange_flat_tree::Builder;
use flange_flat_tree::Tree;
use flange_flat_tree::Subtree;

let mut b = Builder::new();
b.start_element("root");
b.start_end_element("first_child");
b.start_element("second_child_with_children");
b.start_end_element("deep_child");
b.end_element();
b.end_element();

let tree = b.build();

assert_eq!(tree.root().value(), &"root");
assert_eq!(tree.root().children()[0].value(), &"first_child");
assert_eq!(tree.root().children()[1].children()[0].value(), &"deep_child");
```
*/
pub struct Builder<A> {
    nav_builder: crate::navigator::Builder,
    values: Vec<A>,
}

impl<A> Builder<A> {
    pub fn new() -> Builder<A> {
        Builder {
            nav_builder: crate::navigator::Builder::new(),
            values: Vec::new(),
        }
    }

    pub fn with_capacity(c: usize) -> Builder<A> {
        Builder {
            nav_builder: crate::navigator::Builder::with_capacity(c),
            values: Vec::with_capacity(c),
        }
    }

    pub fn get(&self, index: usize) -> Option<&A> {
        self.values.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut A> {
        self.values.get_mut(index)
    }

    pub fn start_element(&mut self, el: A) -> usize {
        self.values.push(el);
        self.nav_builder.start_element()
    }

    pub fn end_element(&mut self) -> usize {
        self.nav_builder.end_element()
    }

    pub fn start_end_element(&mut self, el: A) -> usize {
        self.values.push(el);
        self.nav_builder.start_end_element()
    }
}

impl<A> Builder<A> {
    pub fn build(self) -> VecTree<A> {
        VecTree {
            nav: self.nav_builder.build(),
            values: self.values,
        }
    }
}
