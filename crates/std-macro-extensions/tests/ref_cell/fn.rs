use super::*;

#[test]
fn test_ref_cell() {
    let my_refcell: RefCell<i32> = refcell!(5);
    assert_eq!(*my_refcell.borrow(), 5);
    my_refcell.replace(10);
    assert_eq!(*my_refcell.borrow(), 10);
}
