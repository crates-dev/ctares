use super::*;

#[test]
fn test_vec() {
    let empty_vector: Vec<i32> = vector!();
    assert!(empty_vector.is_empty());
    let numbers: Vec<i32> = vector!(1, 2, 3);
    assert_eq!(numbers.len(), 3);
    assert_eq!(numbers[0], 1);
    assert_eq!(numbers[1], 2);
    assert_eq!(numbers[2], 3);
}
