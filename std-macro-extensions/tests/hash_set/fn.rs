use super::*;

#[test]
fn test_hash_set() {
    let my_set: HashSet<i32> = hash_set!();
    assert!(my_set.is_empty());
    let my_set: HashSet<i32> = hash_set!(1, 2, 3);
    assert!(my_set.contains(&1));
    assert!(my_set.contains(&2));
    assert!(my_set.contains(&3));
}
