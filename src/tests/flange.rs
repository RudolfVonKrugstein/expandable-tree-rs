use crate::TreeBuilder;

#[test]
fn test_flange() {
    let mut builder = TreeBuilder::with_capacity(2);
    builder.start_element(1);
    builder.start_end_element(2);
    builder.end_element();
    let tree = builder.build();

    let values: Vec<u32> = (10..12).collect();
    let tree_with_values = tree.flange(values);
    assert_eq!(tree_with_values.root().value(), (&1, &10));
}

#[test]
fn test_flange_twice() {
    let mut builder = TreeBuilder::with_capacity(2);
    builder.start_element(1);
    builder.start_end_element(2);
    builder.end_element();
    let tree = builder.build();

    let values: Vec<u32> = (10..12).collect();
    let tree_with_values = tree.flange(values);
    let values: Vec<u32> = (13..15).collect();
    let tree_with_values = tree_with_values.flange(values);
    assert_eq!(tree_with_values.root().value(), (&1, &10, &13));
}

#[test]
fn test_ref_flange() {
    let mut builder = TreeBuilder::with_capacity(2);
    builder.start_element(1);
    builder.start_end_element(2);
    builder.end_element();
    let tree = builder.build();

    let values: Vec<u32> = (10..12).collect();
    let tree_with_values = tree.ref_flange(&values);
    assert_eq!(tree.root().value(), &1);
    assert_eq!(tree_with_values.root().value(), (&1, &10));
}
