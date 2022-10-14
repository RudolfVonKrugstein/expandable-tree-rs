use crate::{tree::Tree, Builder, Subtree};

#[test]
fn test_create() {
    // Use builder
    let mut builder = Builder::with_capacity(10);
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
    let _tree = builder.build();
}

#[test]
fn simple_test() {
    // Setup

    // Insert a root and some children
    let mut b = Builder::new();
    b.start_element(0);
    b.start_end_element(1);
    b.start_end_element(2);
    b.start_end_element(3);
    b.end_element();
    let t = b.build();

    let root = t.root();

    // Test
    assert_eq!(*root.value(), 0);
    assert_eq!(root.child_values(), vec![&1, &2, &3]);
}

#[test]
fn multi_children_test() {
    // Insert a root and some children
    let mut t = Builder::new();
    t.start_element(0);
    t.start_element(1);
    t.start_end_element(2);
    t.start_end_element(3);
    t.start_end_element(4);
    t.end_element();
    t.start_end_element(5);
    t.start_end_element(6);
    t.start_end_element(7);
    t.end_element();
    let t = t.build();
    let root = t.root();

    // Test
    assert_eq!(root.child_values(), vec![&1, &5, &6, &7]);
    assert_eq!(root.first_child().unwrap().child_values(), vec![&2, &3, &4]);
    assert_eq!(
        root.first_child()
            .unwrap()
            .first_child()
            .unwrap()
            .next_sibling()
            .unwrap()
            .value(),
        &3
    );
    assert_eq!(
        root.first_child().unwrap().children()[1]
            .next_sibling()
            .unwrap()
            .value(),
        &4
    );
    assert_eq!(
        root.first_child().unwrap().children()[1]
            .prev_sibling()
            .unwrap()
            .value(),
        &2
    );
    assert!(root.first_child().unwrap().children()[2]
        .next_sibling()
        .is_none());
    assert!(root.first_child().unwrap().children()[0]
        .prev_sibling()
        .is_none());
    assert!(root.parent().is_none());
    assert_eq!(root.children()[0].parent().unwrap().value(), &0);
    assert_eq!(root.children()[1].parent().unwrap().value(), &0);
    assert_eq!(root.children()[2].parent().unwrap().value(), &0);
    assert_eq!(root.children()[3].parent().unwrap().value(), &0);
    assert_eq!(
        root.first_child().unwrap().children()[0]
            .parent()
            .unwrap()
            .value(),
        &1
    );
}
