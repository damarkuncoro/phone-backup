use phone_backup_adapter_adb::{AdbClient, AdbWorkerPool, ConcurrentAdbStreamer};

#[test]
fn test_worker_pool_concurrency_and_guards() {
    let client = AdbClient::new();
    let pool = AdbWorkerPool::new(client, 2);

    assert_eq!(pool.max_concurrency(), 2);
    assert_eq!(pool.active_workers(), 0);

    let guard1 = pool.try_acquire();
    assert!(guard1.is_some());
    assert_eq!(pool.active_workers(), 1);

    let guard2 = pool.try_acquire();
    assert!(guard2.is_some());
    assert_eq!(pool.active_workers(), 2);

    // Concurrency limit reached
    let guard3 = pool.try_acquire();
    assert!(guard3.is_none());

    // Dropping releases capacity
    drop(guard1);
    assert_eq!(pool.active_workers(), 1);

    let streamer = ConcurrentAdbStreamer::new(pool);
    assert_eq!(streamer.pool().max_concurrency(), 2);
}
