use super::*;

#[test]
fn test_recoverable_spawn() {
    let msg: &str = "test";
    let res: SyncSpawnResult = recoverable_spawn(move || {
        panic!("{}", msg);
    });
    println!("test_recoverable_spawn handle res {res:?}");
}

#[test]
fn test_recoverable_spawn_catch() {
    let msg: &str = "test";
    let res: SyncSpawnResult = recoverable_spawn_catch(
        move || {
            panic!("{}", msg);
        },
        |err: &str| {
            println!("handle error => {err}");
        },
    );
    println!("test_recoverable_spawn_catch handle res {res:?}");
}

#[test]
fn test_recoverable_spawn_catch_finally() {
    let msg: &str = "test";
    let res: SyncSpawnResult = recoverable_spawn_catch_finally(
        move || {
            panic!("{}", msg);
        },
        |err: &str| {
            println!("handle error => {err}");
            panic!("{}", err);
        },
        || {
            println!("finally");
        },
    );
    println!("test_recoverable_spawn_catch_finally handle res {res:?}");
}
