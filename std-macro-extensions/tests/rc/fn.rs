use super::*;

#[test]
fn test_rc() {
    let my_rc: Rc<i32> = rc!(5);
    assert_eq!(*my_rc, 5);
}
