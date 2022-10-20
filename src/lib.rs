/*!

A Rust implementation of Tree Data-Structure storing nodes in a Flat Array (`Vec` to be exact)
and allowing to "flange" more data to the nodes.

# Examples

## Building a tree.

A tree is build using the `Builder`. Afterwards a tree can not be changed anymore.

```
use flange_flat_tree::{Builder, Tree, Subtree};

let mut b = Builder::new();
b.start_element("one"); // The root node
b.start_end_element("two"); // This will be a child of "one"
b.start_element("three"); // This will be a child of "one"
b.start_end_element("four"); // This will be a child of "three"
b.end_element(); // End "three"
b.end_element(); // End "one" (the root)

let tree = b.build();
assert_eq!(tree.root().value(), &"one");
assert_eq!(tree.root().child_values(), [&"two", &"three"]);
```

But you can change elements of the tree still in the builder by using the index:

```
use flange_flat_tree::{Builder, Tree, Subtree};

struct Node {
    pub name: String,
    pub value: u32,
}

let mut b = Builder::new();
let root_id = b.start_element(Node { name: "one".to_string(), value: 1 }); // The root node
b.start_end_element(Node { name: "two".to_string(), value: 2 }); // This will be a child of "one"

// Change root afterwards
b.get_mut(root_id).unwrap().name = "root".to_string();
b.end_element(); // End "one" (the root)

let tree = b.build();
assert_eq!(tree.root().value().name, "root".to_string());
```

See the [`builder`](./src/tree/builder.rs) for more detail.

## Flanging data

After the tree is build, additional data can be "flanged" to the nodes of the tree.
This means data is added without modifying the original tree or copying its data.

"Flanging" can happen in 2 ways:

### Flanging using "flange"

You can flange using `tree.flange`.
This crates a new tree which references the old tree and adds new data
from the vector passed to `tree.flange`. The Vector is consumed, so it is
owned by the new tree.

The nodes of the `Vec` are associated the nodes with the same index in the
original tree. When you access the tree you get a tuple of references to the values:

```
use flange_flat_tree::{Builder, Tree, Subtree};

let mut builder = Builder::with_capacity(2);
builder.start_element("one".to_string());
builder.start_end_element("two".to_string());
builder.end_element();
let tree = builder.build();

// flanged values
let values: Vec<u32> = (10..12).collect();

let tree_with_values = tree.flange(values);
assert_eq!(tree_with_values.root().value(), (&"one".to_string(), &10));
```

You can also create the flange by directly mapping from the original tree:

```
use flange_flat_tree::{Builder, Tree, Subtree};

let mut builder = Builder::with_capacity(2);
builder.start_element("one".to_string());
builder.start_end_element("two".to_string());
builder.end_element();
let tree = builder.build();

let tree_with_values = tree.flange_map(
    |n| format!("{}-flanged", n)
);

assert_eq!(tree_with_values.root().value(), (&"one".to_string(), &"one-flanged".to_string()));
```

This can also be done "depth-first", in which case the children are available before the node is created:

```
use flange_flat_tree::{Builder, Tree, Subtree};

let mut builder = Builder::with_capacity(2);
builder.start_element("one".to_string());
builder.start_end_element("two".to_string());
builder.end_element();
let tree = builder.build();

let tree_with_values = tree.depth_first_flange(
    |n, childs| format!("{} with {} children", n, childs.len())
);

assert_eq!(tree_with_values.root().value(), (&"one".to_string(), &"one with 1 children".to_string()));
assert_eq!(tree_with_values.root().children()[0].value(), (&"two".to_string(), &"two with 0 children".to_string()));
```

### Flange using map

If you want to access the tree and its flange using named properties, you have to create
a type for it. The type should reference the values it exposes. Than
you can use `tree.map` with a closure to return your type.
You can also combine the existing tree with external data that way:

```
use flange_flat_tree::{Builder, Tree, Subtree};

let mut builder = Builder::with_capacity(2);
builder.start_element("one");
builder.start_end_element("two");
builder.end_element();
let tree = builder.build();

struct FlangedNode<'a> {
    pub name: &'a str,
    pub value: u32, // we don't use a reference for u32, because copying the value itself is fine
}

// The data we are going to flange
let data: Vec<u32> = vec![1,2];

let tree_with_values = tree.map(
    |i,v| FlangedNode {
        name: v,
        value: data[i],
    }
);

assert_eq!(tree_with_values.root().value().name, "one");
assert_eq!(tree_with_values.root().value().value, 1);
```
*/

mod navigator;
mod tree;

pub use navigator::Neighbors;
pub use tree::Builder;
pub use tree::FlangedTree;
pub use tree::Subtree;
pub use tree::Tree;
pub use tree::VecTree;

#[cfg(test)]
mod tests;
