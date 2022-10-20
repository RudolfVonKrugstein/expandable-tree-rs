use crate::navigator::Navigator;
use crate::{FlangedTree, Subtree};

use super::tree_data::TreeData;
use super::MappedTree;

/**
Trait for everything that can be handled as a tree.
*/
pub trait Tree<'a>: Sized
where
    Self: 'a,
    &'a Self: TreeData<Node = Self::Node>,
{
    /// The type of the trees nodes
    type Node;
    /// The type of the subtree, returned by `root()`
    type SubtreeType: Subtree<Node = Self::Node>;

    /// Direct access to a node in the tree via its position in the flat map
    fn at_pos(&'a self, index: usize) -> Self::SubtreeType;

    /// Direct access to the `Navigator` storing the neighboring information of the tree
    fn get_nav(&self) -> &Navigator;

    /// The root node of the tree.
    fn root(&'a self) -> Self::SubtreeType {
        self.at_pos(0)
    }

    /// The number of nodes in the tree.
    fn node_count(&'a self) -> usize {
        self.count()
    }

    /** Create a new tree using a function to map from the nodes of this tree.
    The map function can also include external data sources.

    # Example
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
    fn map<B, M>(&'a self, m: M) -> MappedTree<Self::Node, B, M, &'a Self>
    where
        M: Fn(usize, Self::Node) -> B,
    {
        MappedTree::new(self, m)
    }

    /** Flange data to the nodes.
    A new tree is created, that references the old tree and whos
    `Node` type is a typle with a reference to the old data
    and a reference to the new data from the inserted `data` vector.
    */
    fn flange<B>(&'a self, data: Vec<B>) -> FlangedTree<&'a Self, B> {
        FlangedTree::new(self, data)
    }

    /** Use `flange`, but create the data using the `mapf` function*/
    fn flange_map<B,F>(&'a self, mapf: F) -> FlangedTree<&'a Self, B>
    where
        B: 'a,
        B: Clone,
        F: Fn(Self::Node) -> B {
        let mut res = Vec::with_capacity(self.node_count());

        for index in 0..self.count() {
            let new_val = mapf(self.get(index));
            res.push(
                new_val
            );
        }
        FlangedTree::new(self, res)
    }

    fn for_each<F>(&'a self, mut f: F)
    where
        F: FnMut(Self::SubtreeType),
    {
        for i in 0..self.count() {
            f(self.at_pos(i))
        }
    }

    /** Flange data to the nodes using a map function in a depth first order.

    This means the children are available (already created) when a node
    is created and can be used to calculate the nodes value.

    # Example
    ```
    use flange_flat_tree::{Builder, Tree, Subtree};

    let mut builder = Builder::with_capacity(2);
    builder.start_element("one");
    builder.start_element("two");
    builder.start_end_element("three");
    builder.start_end_element("four");
    builder.end_element();
    builder.end_element();
    let tree = builder.build();

    // The data we are going to flange
    let data: Vec<u32> = vec![1,2];

    let tree_with_values = tree.depth_first_flange(
        |v, childs| {
            // Calculate the total number of children
            childs.iter().fold(childs.len(), |l,c| l+*c )
        }
    );

    assert_eq!(tree_with_values.root().value().1, &3);
    assert_eq!(tree_with_values.root().children()[0].value(), (&"two", &2));
    ```
     */
    fn depth_first_flange<B, F>(&'a self, mapf: F) -> FlangedTree<&'a Self, B>
    where
        B: 'a,
        B: Default,
        B: Clone,
        F: Fn(Self::Node, Vec<&B>) -> B,
    {
        // Uninitialized vector
        let mut res = vec![B::default(); self.node_count()];

        self.get_nav().for_each_depth_first(|i, childs| {
            let new_val = mapf(self.get(i), childs.iter().map(|&c| &res[c]).collect());
            res[i] = new_val;
        });
        FlangedTree::new(self, res)
    }
}

// impl<A: Sized, Node> Tree for A
// where
//     for<'a> &'a A: TreeData<Node = &'a Node>,
//     for<'a> Node: 'a,
// {
//     fn root(&self) -> SubtreeImpl<A> {
//         SubtreeImpl::new(self, 0)
//     }
// }
