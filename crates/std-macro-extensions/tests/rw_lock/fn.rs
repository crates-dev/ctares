use super::*;

#[test]
fn test_rw_lock() {
    let my_rwlock: RwLock<i32> = rw_lock!(5);
    assert_eq!(*my_rwlock.read().unwrap(), 5);
    *my_rwlock.write().unwrap() = 10;
    assert_eq!(*my_rwlock.read().unwrap(), 10);
}
