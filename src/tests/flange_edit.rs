use crate::{tree::Tree, Builder, Subtree};

#[test]
fn test_flange_edit() {
    let mut builder = Builder::with_capacity(2);
    builder.start_element("one".to_string());
    builder.start_end_element("two".to_string());
    builder.end_element();
    let tree = builder.build();

    let values: Vec<u32> = (10..12).collect();
    let mut tree_with_values = tree.flange(values);
    *tree_with_values.get_flange_mut(0) = 9;
    assert_eq!(tree_with_values.root().value(), (&"one".to_string(), &9));
}

#[test]
fn test_flange_complex_edit() {
    let mut builder = Builder::with_capacity(2);
    builder.start_element("one".to_string());
    builder.start_end_element("two".to_string());
    builder.end_element();
    let tree = builder.build();
    let mut tree_with_values =
        tree.depth_first_flange(|value, _children| format!("{}flanged", value));
    *tree_with_values.get_flange_mut(0) = "1flanged".to_string();
    assert_eq!(
        tree_with_values.root().value(),
        (&"one".to_string(), &"1flanged".to_string())
    );
}
