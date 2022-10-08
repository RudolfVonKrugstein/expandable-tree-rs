use crate::Builder;

#[test]
fn test_reduce() {
    let mut builder = Builder::with_capacity(2);
    builder.start_element(1);
    builder.start_end_element(2);
    builder.end_element();
    let tree = builder.build();

    let values: Vec<u32> = (10..12).collect();
    let tree_with_values = tree.flange(values);
    let (tree, values) = tree_with_values.un_flange();
    assert_eq!(tree.root().value(), &1);
    assert_eq!(values[0], 10);
}
