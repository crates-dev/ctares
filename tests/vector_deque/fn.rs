use super::*;

#[test]
fn test_vector_deque() {
    let empty_deque: VecDeque<i32> = vector_deque!();
    assert!(empty_deque.is_empty());
    let numbers: VecDeque<i32> = vector_deque!(1, 2, 3);
    assert_eq!(numbers.len(), 3);
    assert_eq!(numbers[0], 1);
    assert_eq!(numbers[1], 2);
    assert_eq!(numbers[2], 3);
}
