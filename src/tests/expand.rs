use crate::TreeBuilder;

#[test]
fn test_expand() {
    let mut builder = TreeBuilder::with_capacity(2);
    builder.start_element(1);
    builder.start_end_element(2);
    builder.end_element();
    let tree = builder.build();

    let values: Vec<u32> = (10..12).collect();
    let tree_with_values = tree.expand(values);
    assert_eq!(tree_with_values.root().value(), (&1, &10));
}

#[test]
fn test_expand_twice() {
    let mut builder = TreeBuilder::with_capacity(2);
    builder.start_element(1);
    builder.start_end_element(2);
    builder.end_element();
    let tree = builder.build();

    let values: Vec<u32> = (10..12).collect();
    let tree_with_values = tree.expand(values);
    let values: Vec<u32> = (13..15).collect();
    let tree_with_values = tree_with_values.expand(values);
    assert_eq!(tree_with_values.root().value(), (&1, &10, &13));
}
