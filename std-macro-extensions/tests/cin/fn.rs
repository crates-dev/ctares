use super::*;

#[test]
fn test_cin_parse() {
    let input: &str = "1 2 3";
    let numbers: Vec<i32> = cin_parse!(input, Vec<i32>);
    assert_eq!(numbers, vec![1, 2, 3]);
    let single_input: &str = "12";
    let number: i32 = cin_parse!(single_input, i32);
    assert_eq!(number, 12);
}
