use super::*;

#[test]
fn test_box() {
    let boxed_value: Box<i32> = boxed!(10);
    assert_eq!(boxed_value, Box::new(10));
}
