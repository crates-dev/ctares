use super::*;

use std::sync::Arc;
#[test]
fn test() {
    let thread_pool: ThreadPool = ThreadPool::new(1);
    let first_res: SendResult = thread_pool.execute(|| {
        println!("first");
    });
    println!("{first_res:?}");
    let panic_res: SendResult = thread_pool.execute_with_catch(
        || {
            panic!("[panic]");
        },
        |err: &str| {
            println!("Catch panic {err}");
        },
    );
    println!("{panic_res:?}");
    let second_res: SendResult = thread_pool.execute_with_catch_finally(
        || {
            panic!("[panic]");
        },
        |_err| {
            panic!("[panic]");
        },
        || {
            println!("finally");
        },
    );
    println!("{second_res:?}");
    sleep(Duration::from_secs(10));
}

#[tokio::test(flavor = "multi_thread")]
async fn async_test() {
    let thread_pool: ThreadPool = ThreadPool::new(1);
    let first_res: SendResult = thread_pool.async_execute(|| async {
        println!("first");
    });
    println!("{first_res:?}");
    let panic_res: SendResult = thread_pool.async_execute_with_catch(
        || async {
            panic!("[panic]");
        },
        |err: Arc<String>| async move {
            println!("Catch panic {err}");
        },
    );
    println!("{panic_res:?}");
    let second_res: SendResult = thread_pool.async_execute_with_catch_finally(
        || async {
            panic!("[panic]");
        },
        |_err| async {
            panic!("[panic]");
        },
        || async {
            println!("finally");
        },
    );
    println!("{second_res:?}");
    sleep(Duration::from_secs(10));
}
