# Flange-FLat-Tree

A Rust implementation of Tree Data-Structure storing nodes in a Flat Array (`Vec` to be exact)
and allowing to "flange" more data to the nodes.

## Flat storing of node data

The `FlatTree` stores the data for the nodes in a `Vec`.
That this is the case is not visible to the user, so you may know it but it does not change how you use the tree.

## Building a tree.

A tree is build using the `Buidler`. Afterwards a tree can not be changed anymore.
This is not a restriction which is impossible to change, its just implemented that way right now
(you are free to make a pull request ;)).

See the [`builder`](./src/tree/builder.rs) module for details.

## Flanging data

After the tree is build, additional data can be "flanged" to the nodes of the tree.
This means data is added without modifying the original tree or copying its data.

"Flanging" can happen in 2 ways:

* By consuming the original tree and creating a new "flanged" tree.
  This happens without deep-copying the node data, but the original tree can not be
  used anymore (it is moved into the new tree).
  You can not do this, when you just have a borrow of a tree, you must own the tree.
* By referencing the original tree.
  That creates a new tree which references (borrows) the old tree.
  This is useful if you do not own the tree (you just have a reference to it).


