use crate::TreeBuilder;

#[test]
fn test_create() {
    // Use builder
    let mut builder = TreeBuilder::with_capacity(10);
    builder.start_element(1);
    builder.start_end_element(2);
    builder.start_end_element(3);
    builder.start_end_element(4);
    builder.start_element(5);
    builder.start_end_element(6);
    builder.start_end_element(7);
    builder.start_element(8);
    builder.start_end_element(9);
    builder.end_element();
    builder.start_end_element(10);
    builder.end_element();
    builder.end_element();
    let tree = builder.build();
}
