use super::*;

#[test]
fn test_b_tree_set() {
    let mut b_tree_set_b: BTreeSet<&str> = b_tree_set!();
    let b_tree_set_a: BTreeSet<&str> = b_tree_set!("a", "b");
    b_tree_set_b.insert("a");
    b_tree_set_b.insert("b");
    assert_eq!(b_tree_set_a, b_tree_set_b);
}
