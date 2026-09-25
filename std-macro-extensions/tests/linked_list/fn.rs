use super::*;

#[test]
fn test_linked_test() {
    let my_list: LinkedList<i32> = linked_list!();
    assert!(my_list.is_empty());
    let my_list: LinkedList<i32> = linked_list!(1, 2, 3);
    assert_eq!(my_list.len(), 3);
    assert_eq!(my_list.front(), Some(&1));
    assert_eq!(my_list.back(), Some(&3));
}
