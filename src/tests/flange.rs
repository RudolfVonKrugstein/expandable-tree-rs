use crate::{tree::Tree, Builder, Subtree};

#[test]
fn test_flange() {
    let mut builder = Builder::with_capacity(2);
    builder.start_element("one".to_string());
    builder.start_end_element("two".to_string());
    builder.end_element();
    let tree = builder.build();

    let values: Vec<u32> = (10..12).collect();
    let tree_with_values = tree.flange(&values);
    assert_eq!(tree_with_values.root().value(), (&"one".to_string(), &10));
}

// #[test]
// fn test_flange_twice() {
//     let mut builder = Builder::with_capacity(2);
//     builder.start_element(1);
//     builder.start_end_element(2);
//     builder.end_element();
//     let tree = builder.build();
//
//     let values: Vec<u32> = (10..12).collect();
//     let tree_with_values = FlangedTree::new(&tree, &values);
//     let values: Vec<u32> = (13..15).collect();
//     let tree_with_values = FlangedTree::new(&tree_with_values, &values);
//     assert_eq!(tree_with_values.root().value(), (&1, &10, &13));
// }
