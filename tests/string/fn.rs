use super::*;

#[test]
fn test_string() {
    let empty_string: String = string!();
    assert!(empty_string.is_empty());
    let hello_string: String = string!("Hello");
    assert_eq!(hello_string, "Hello");
}
