use crate::{
    tree::{MappedTree, Subtree},
    Builder, Tree,
};

#[derive(Debug, PartialEq)]
struct MultValNode<'a> {
    pub v1: &'a u32,
    pub v2: &'a u32,
}

#[test]
fn test_subtree() {
    let mut builder = Builder::with_capacity(2);
    builder.start_element(1);
    builder.start_end_element(2);
    builder.end_element();
    let tree = builder.build();

    fn print_subtree<'a, ST>(st: ST)
    where
        ST: Subtree<Node = u32>,
    {
        println!("{}", st.value())
    }

    let values: Vec<u32> = (10..12).collect();
    let tree_with_values = MappedTree::new(&tree, |i, n| MultValNode {
        v1: n,
        v2: &values[i],
    });
    // let tree_with_values = tree.flange(values);
    // let (tree, values) = tree_with_values.un_flange();
    assert_eq!(tree.root().value(), &1);
    assert_eq!(tree_with_values.root().value().v1, &1);
    assert_eq!(tree_with_values.root().value().v2, &10);
    assert_eq!(values[0], 10);
}
