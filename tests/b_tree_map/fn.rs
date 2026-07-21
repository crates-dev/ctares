use super::*;

#[test]
fn test_b_tree_map() {
    let _empty_map: BTreeMap<i32, i32> = b_tree_map!();
    let b_tree_map_a: BTreeMap<&str, &str> = b_tree_map!("a" => "a","b" => "b");
    let b_tree_map_b: BTreeMap<&str, &str> = b_tree_map!("a" => "a","b" => "b");
    assert_eq!(b_tree_map_a, b_tree_map_b);
}
