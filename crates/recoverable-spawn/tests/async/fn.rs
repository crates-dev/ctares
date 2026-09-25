use super::*;

use std::sync::Arc;

#[tokio::test]
async fn test_async_recoverable_spawn() {
    let msg: &str = "test";
    let res: AsyncSpawnResult = async_recoverable_spawn(move || async move {
        panic!("{}", msg);
    })
    .await;
    println!("test_async_recoverable_spawn handle res {res:?}");
}

#[tokio::test]
async fn test_async_recoverable_spawn_catch() {
    let msg: &str = "test";
    let res: AsyncSpawnResult = async_recoverable_spawn_catch(
        move || async move {
            panic!("{}", msg);
        },
        move |err: Arc<String>| async move {
            println!("async handle error => {err}");
        },
    )
    .await;
    println!("test_async_recoverable_spawn_catch handle res {res:?}");
}

#[tokio::test]
async fn test_async_recoverable_spawn_catch_finally() {
    let msg: &str = "test";
    let res: AsyncSpawnResult = async_recoverable_spawn_catch_finally(
        move || async move {
            panic!("{}", msg);
        },
        move |err: Arc<String>| async move {
            println!("async handle error => {err}");
            panic!("{}", err);
        },
        move || async move {
            println!("finally");
        },
    )
    .await;
    println!("test_async_recoverable_spawn_catch_finally handle res {res:?}");
}
