use super::*;

#[test]
fn public_storage_alias_retains_its_original_hasher() {
    let map: DashMapStringBroadcast<u64> = DashMapStringBroadcast::default();
    let _: &BuildHasherDefault<XxHash3_64> = map.hasher();
}

#[tokio::test]
async fn long_and_unicode_keys_remain_isolated_across_clones() {
    let map: BroadcastMap<Vec<u8>> = BroadcastMap::default();
    let keys: Vec<String> = vec![
        String::new(),
        "short".to_owned(),
        "x".repeat(241),
        "y".repeat(4096),
        "topic/连接".repeat(64),
    ];
    let mut receivers: Vec<BroadcastMapReceiver<Vec<u8>>> = keys
        .iter()
        .map(|key: &String| map.subscribe_or_insert(key, 64))
        .collect();
    let cloned: BroadcastMap<Vec<u8>> = map.clone();
    for (index, key) in keys.iter().enumerate() {
        assert_eq!(map.try_send(key, vec![index as u8]).unwrap(), Some(1));
        assert_eq!(cloned.try_send(key, vec![index as u8, 1]).unwrap(), Some(1));
    }
    for (index, receiver) in receivers.iter_mut().enumerate() {
        assert_eq!(receiver.recv().await.unwrap(), vec![index as u8]);
        assert_eq!(receiver.recv().await.unwrap(), vec![index as u8, 1]);
        assert!(receiver.try_recv().is_err());
    }
}

#[tokio::test]
async fn test_broadcast_map() {
    let broadcast_map: BroadcastMap<u128> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    let mut rec1: BroadcastMapReceiver<u128> = broadcast_map.subscribe("test_key").unwrap();
    let mut rec2: BroadcastMapReceiver<u128> = broadcast_map.subscribe("test_key").unwrap();
    let mut rec3: BroadcastMapReceiver<u128> =
        broadcast_map.subscribe_or_insert("another_key", DEFAULT_BROADCAST_SENDER_CAPACITY);
    broadcast_map.send("test_key", 20).unwrap();
    broadcast_map.send("another_key", 10).unwrap();
    assert_eq!(rec1.recv().await, Ok(20));
    assert_eq!(rec2.recv().await, Ok(20));
    assert_eq!(rec3.recv().await, Ok(10));
}

#[tokio::test]
async fn test_broadcast_map_unsubscribe() {
    let broadcast_map: BroadcastMap<u128> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    let mut rec1: BroadcastMapReceiver<u128> = broadcast_map.subscribe("test_key").unwrap();
    let removed: Option<Broadcast<u128>> = broadcast_map.unsubscribe("test_key");
    assert!(removed.is_some());
    drop(removed);
    let not_exist: Option<Broadcast<u128>> = broadcast_map.unsubscribe("nonexistent_key");
    assert!(not_exist.is_none());
    assert!(broadcast_map.subscribe("test_key").is_none());
    let send_result: Result<Option<ReceiverCount>, SendError<u128>> =
        broadcast_map.try_send("test_key", 30);
    assert!(send_result.unwrap().is_none());
    let result: Result<Result<u128, RecvError>, Elapsed> =
        timeout(Duration::from_millis(100), rec1.recv()).await;
    assert!(result.is_ok(), "recv should not timeout after unsubscribe");
    assert_eq!(result.unwrap(), Err(RecvError::Closed));
}

#[tokio::test]
async fn test_broadcast_map_unsubscribe_and_reinsert() {
    let broadcast_map: BroadcastMap<u128> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    broadcast_map.subscribe("test_key").unwrap();
    let removed: Option<Broadcast<u128>> = broadcast_map.unsubscribe("test_key");
    assert!(removed.is_some());
    broadcast_map.insert("test_key", 10);
    let mut rec2: BroadcastMapReceiver<u128> = broadcast_map.subscribe("test_key").unwrap();
    broadcast_map.send("test_key", 100).unwrap();
    assert_eq!(rec2.recv().await, Ok(100));
}

#[tokio::test]
async fn test_broadcast_map_unsubscribe_receiver_count() {
    let broadcast_map: BroadcastMap<String> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    let _rec1: BroadcastMapReceiver<String> = broadcast_map.subscribe("test_key").unwrap();
    let _rec2: BroadcastMapReceiver<String> = broadcast_map.subscribe("test_key").unwrap();
    assert_eq!(broadcast_map.receiver_count("test_key"), Some(2));
    let removed: Option<Broadcast<String>> = broadcast_map.unsubscribe("test_key");
    assert!(removed.is_some());
    assert_eq!(broadcast_map.receiver_count("test_key"), None);
}

#[tokio::test]
async fn test_broadcast_map_send() {
    let broadcast_map: BroadcastMap<u128> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    let mut rec1: BroadcastMapReceiver<u128> = broadcast_map.subscribe("test_key").unwrap();
    let mut rec2: BroadcastMapReceiver<u128> = broadcast_map.subscribe("test_key").unwrap();
    let count: Option<ReceiverCount> = broadcast_map.send("test_key", 42);
    assert_eq!(count, Some(2));
    assert_eq!(rec1.recv().await, Ok(42));
    assert_eq!(rec2.recv().await, Ok(42));
    let non_existent: Option<ReceiverCount> = broadcast_map.send("non_existent_key", 100);
    assert_eq!(non_existent, None);
}

#[test]
fn test_subscribe_or_insert_is_atomic_for_concurrent_first_subscribers() {
    let map: Arc<BroadcastMap<u64>> = Arc::new(BroadcastMap::new());
    for round in 0..64 {
        let barrier: Arc<Barrier> = Arc::new(Barrier::new(16));
        let key: String = format!("race-{round}");
        let mut receivers: Vec<BroadcastMapReceiver<u64>> =
            thread::scope(|scope: &thread::Scope<'_, '_>| {
                let jobs: Vec<thread::ScopedJoinHandle<'_, BroadcastMapReceiver<u64>>> = (0..16)
                    .map(|_| {
                        let map: Arc<BroadcastMap<u64>> = map.clone();
                        let barrier: Arc<Barrier> = barrier.clone();
                        let key: &str = &key;
                        scope.spawn(move || {
                            barrier.wait();
                            map.subscribe_or_insert(key, 64)
                        })
                    })
                    .collect();
                jobs.into_iter()
                    .map(
                        |job: thread::ScopedJoinHandle<'_, BroadcastMapReceiver<u64>>| {
                            job.join().unwrap()
                        },
                    )
                    .collect()
            });
        assert_eq!(map.receiver_count(&key), Some(16));
        assert_eq!(map.try_send(&key, round).unwrap(), Some(16));
        for receiver in &mut receivers {
            assert_eq!(receiver.try_recv(), Ok(round));
        }
        assert!(map.unsubscribe(&key).is_some());
    }
}

#[tokio::test]
async fn test_existing_channel_keeps_capacity_and_pending_receivers() {
    let map: BroadcastMap<u64> = BroadcastMap::new();
    let mut first: BroadcastMapReceiver<u64> = map.subscribe_or_insert("existing", 1);
    let mut second: BroadcastMapReceiver<u64> = map.subscribe_or_insert("existing", 1024);
    assert_eq!(map.try_send("existing", 1).unwrap(), Some(2));
    assert_eq!(map.try_send("existing", 2).unwrap(), Some(2));
    assert_eq!(first.recv().await, Err(RecvError::Lagged(1)));
    assert_eq!(second.recv().await, Err(RecvError::Lagged(1)));
    assert_eq!(first.recv().await, Ok(2));
    assert_eq!(second.recv().await, Ok(2));
}
