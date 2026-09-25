use super::*;

#[test]
fn test_cout() {
    let name: &str = "Alice";
    let age: i32 = 30;
    cout!("Name: {}, Age: {}\n", name, age);
}

#[test]
fn test_endl() {
    endl!();
}

#[test]
fn test_cout_endl() {
    let name: &str = "Alice";
    let age: i32 = 30;
    cout_endl!("Name: {}, Age: {}\n", name, age);
}
