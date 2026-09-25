use super::*;

#[test]
fn test_empty_binary_heap() {
    let heap: BinaryHeap<i32> = binary_heap!();
    assert!(heap.is_empty());
}

#[test]
fn test_binary_heap_with_elements() {
    let heap: BinaryHeap<i32> = binary_heap!(5, 3, 8, 1);
    let vec: Vec<i32> = heap.into_sorted_vec();
    assert_eq!(vec, vec![1, 3, 5, 8]);
}
