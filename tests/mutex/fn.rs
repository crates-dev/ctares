use super::*;

#[test]
fn test_mutex() {
    let my_mutex: Mutex<i32> = mutex!(5);
    let lock: MutexGuard<'_, i32> = my_mutex.lock().unwrap();
    assert_eq!(*lock, 5);
}
